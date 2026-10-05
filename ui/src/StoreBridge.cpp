#include "StoreBridge.h"

#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonParseError>
#include <QLocalSocket>
#include <QTimer>

#include <memory>

namespace {
constexpr qsizetype maxLine = 1024 * 1024;
constexpr int callTimeoutMilliseconds = 5000;

struct Pending {
    QLocalSocket *socket = nullptr;
    QTimer *timer = nullptr;
    QByteArray bytes;
    QString requestId;
    QString operation;
    bool done = false;
};

bool isSnapshot(const QJsonObject &result) {
    const auto catalogue = result.value(QStringLiteral("catalogue")).toObject();
    const auto version = catalogue.value(QStringLiteral("schemaVersion")).toInt();
    return (version == 1 || version == 2)
           && catalogue.value(QStringLiteral("entries")).isArray()
           && result.value(QStringLiteral("jobs")).isArray()
           && result.value(QStringLiteral("installed")).isArray()
           && result.value(QStringLiteral("backends")).isObject();
}
}

StoreBridge::StoreBridge(QString socketPath, QObject *parent)
    : QObject(parent), socketPath_(std::move(socketPath)), pollTimer_(new QTimer(this)) {
    pollTimer_->setInterval(1000);
    connect(pollTimer_, &QTimer::timeout, this, &StoreBridge::refresh);
}

void StoreBridge::startPolling() {
    refresh();
    pollTimer_->start();
}

void StoreBridge::refresh() {
    if (snapshotPending_) return;
    snapshotPending_ = true;
    send(QStringLiteral("snapshot"), {});
}

void StoreBridge::enqueue(const QString &appId, const QString &action) {
    send(QStringLiteral("enqueue"), {{QStringLiteral("appId"), appId}, {QStringLiteral("action"), action}});
}

void StoreBridge::cancel(const QString &jobId) {
    send(QStringLiteral("cancel"), {{QStringLiteral("jobId"), jobId}});
}

void StoreBridge::retry(const QString &jobId) {
    send(QStringLiteral("retry"), {{QStringLiteral("jobId"), jobId}});
}

void StoreBridge::setError(const QString &message) {
    const QString safe = message.left(240);
    if (error_ == safe) return;
    error_ = safe;
    emit errorChanged();
}

void StoreBridge::send(const QString &operation, const QVariantMap &payload) {
    auto *socket = new QLocalSocket(this);
    auto *timer = new QTimer(socket);
    timer->setSingleShot(true);
    auto pending = std::make_shared<Pending>();
    pending->socket = socket;
    pending->timer = timer;
    pending->operation = operation;
    pending->requestId = QStringLiteral("ui-%1").arg(nextRequest_++);

    auto finish = [this, pending](bool success, const QString &reason) {
        if (pending->done) return;
        pending->done = true;
        pending->timer->stop();
        if (pending->operation == QStringLiteral("snapshot")) snapshotPending_ = false;
        if (success) {
            setError({});
        } else {
            setError(reason.isEmpty() ? QStringLiteral("serviceUnavailable") : reason);
        }
        if (pending->operation != QStringLiteral("snapshot")) {
            emit operationFinished(pending->operation, success);
            if (success) refresh();
        }
        pending->socket->abort();
        pending->socket->deleteLater();
    };

    const QJsonObject request{{QStringLiteral("schemaVersion"), 1},
                              {QStringLiteral("requestId"), pending->requestId},
                              {QStringLiteral("operation"), operation},
                              {QStringLiteral("payload"), QJsonObject::fromVariantMap(payload)}};
    const QByteArray wire = QJsonDocument(request).toJson(QJsonDocument::Compact) + '\n';
    if (wire.size() > maxLine) {
        finish(false, QStringLiteral("requestTooLarge"));
        return;
    }
    connect(socket, &QLocalSocket::connected, this, [socket, wire]() { socket->write(wire); });
    connect(socket, &QLocalSocket::readyRead, this, [this, pending, finish]() {
        if (pending->done) return;
        pending->bytes += pending->socket->readAll();
        if (pending->bytes.size() > maxLine) {
            finish(false, QStringLiteral("responseTooLarge"));
            return;
        }
        const qsizetype newline = pending->bytes.indexOf('\n');
        if (newline < 0) return;
        QJsonParseError parseError;
        const auto document = QJsonDocument::fromJson(pending->bytes.left(newline), &parseError);
        if (parseError.error != QJsonParseError::NoError || !document.isObject()) {
            finish(false, QStringLiteral("responseInvalid"));
            return;
        }
        const QJsonObject reply = document.object();
        if (reply.value(QStringLiteral("schemaVersion")).toInt() != 1
            || reply.value(QStringLiteral("requestId")).toString() != pending->requestId
            || !reply.value(QStringLiteral("ok")).isBool()) {
            finish(false, QStringLiteral("responseMismatch"));
            return;
        }
        if (!reply.value(QStringLiteral("ok")).toBool()) {
            const QString reason = reply.value(QStringLiteral("error")).toObject()
                                       .value(QStringLiteral("message")).toString();
            finish(false, reason);
            return;
        }
        if (!reply.value(QStringLiteral("result")).isObject()) {
            finish(false, QStringLiteral("resultInvalid"));
            return;
        }
        const QJsonObject result = reply.value(QStringLiteral("result")).toObject();
        if (pending->operation == QStringLiteral("snapshot")) {
            if (!isSnapshot(result)) {
                finish(false, QStringLiteral("snapshotIncomplete"));
                return;
            }
            snapshot_ = result.toVariantMap();
            emit snapshotChanged();
        }
        finish(true, {});
    });
    connect(socket, &QLocalSocket::disconnected, this, [this, pending, finish]() {
        if (!pending->done) finish(false, QStringLiteral("serviceDisconnected"));
    });
    connect(socket, &QLocalSocket::errorOccurred, this, [this, pending, finish]() {
        if (!pending->done) finish(false, QStringLiteral("serviceUnavailable"));
    });
    connect(timer, &QTimer::timeout, this, [finish]() { finish(false, QStringLiteral("requestTimedOut")); });
    timer->start(callTimeoutMilliseconds);
    socket->connectToServer(socketPath_);
}
