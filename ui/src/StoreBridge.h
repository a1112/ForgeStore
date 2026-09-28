#pragma once

#include <QObject>
#include <QString>
#include <QVariantMap>

class QTimer;

class StoreBridge final : public QObject {
    Q_OBJECT
    Q_PROPERTY(QVariantMap snapshot READ snapshot NOTIFY snapshotChanged)
    Q_PROPERTY(QString error READ error NOTIFY errorChanged)

public:
    explicit StoreBridge(QString socketPath, QObject *parent = nullptr);

    [[nodiscard]] QVariantMap snapshot() const { return snapshot_; }
    [[nodiscard]] QString error() const { return error_; }

    Q_INVOKABLE void refresh();
    Q_INVOKABLE void enqueue(const QString &appId, const QString &action);
    Q_INVOKABLE void cancel(const QString &jobId);
    Q_INVOKABLE void retry(const QString &jobId);
    void startPolling();

signals:
    void snapshotChanged();
    void errorChanged();
    void operationFinished(const QString &operation, bool succeeded);

private:
    void send(const QString &operation, const QVariantMap &payload);
    void setError(const QString &message);

    QString socketPath_;
    QVariantMap snapshot_;
    QString error_;
    quint64 nextRequest_ = 1;
    bool snapshotPending_ = false;
    QTimer *pollTimer_ = nullptr;
};
