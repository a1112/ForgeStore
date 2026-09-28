#include "StoreBridge.h"

#include <QJsonDocument>
#include <QJsonObject>
#include <QLocalServer>
#include <QLocalSocket>
#include <QSignalSpy>
#include <QTemporaryDir>
#include <QtTest>

class StoreBridgeTest final : public QObject {
    Q_OBJECT

private slots:
    void snapshotUsesPrivateJsonLineAndPublishesState();
    void rejectsMismatchedReplyWithoutPublishingState();
    void enqueuesThenRefreshesRealState();
    void unreachableServiceReportsTranslatableErrorCode();
};

static QJsonObject receiveRequest(QLocalServer &server, QLocalSocket *&peer) {
    if (!server.hasPendingConnections()) {
        QSignalSpy connected(&server, &QLocalServer::newConnection);
        if (!connected.wait(2000)) {
            QTest::qFail("bridge did not connect", __FILE__, __LINE__);
            return {};
        }
    }
    peer = server.nextPendingConnection();
    if (peer == nullptr) {
        QTest::qFail("missing accepted connection", __FILE__, __LINE__);
        return {};
    }
    if (!peer->canReadLine()) {
        if (!peer->waitForReadyRead(2000)) {
            QTest::qFail("bridge did not send a request line", __FILE__, __LINE__);
            return {};
        }
    }
    const auto line = peer->readLine();
    if (!line.endsWith('\n')) {
        QTest::qFail("request lacked newline terminator", __FILE__, __LINE__);
        return {};
    }
    return QJsonDocument::fromJson(line).object();
}

static void answer(QLocalSocket *peer, const QJsonObject &request, const QJsonObject &result) {
    const auto response = QJsonObject{{"schemaVersion", 1}, {"requestId", request.value("requestId")},
                                      {"ok", true}, {"result", result}};
    peer->write(QJsonDocument(response).toJson(QJsonDocument::Compact) + '\n');
    QVERIFY(peer->waitForBytesWritten(2000));
}

void StoreBridgeTest::snapshotUsesPrivateJsonLineAndPublishesState() {
    QTemporaryDir directory;
    QVERIFY(directory.isValid());
    QLocalServer server;
    const QString path = directory.filePath("store.sock");
    QVERIFY(server.listen(path));
    StoreBridge bridge(path);
    QSignalSpy changed(&bridge, &StoreBridge::snapshotChanged);
    bridge.refresh();
    QLocalSocket *peer = nullptr;
    const auto request = receiveRequest(server, peer);
    QCOMPARE(request.value("schemaVersion").toInt(), 1);
    QCOMPARE(request.value("operation").toString(), QStringLiteral("snapshot"));
    QCOMPARE(request.value("payload").toObject(), QJsonObject{});
    QVERIFY(!request.value("requestId").toString().isEmpty());
    answer(peer, request, QJsonObject{{"catalogue", QJsonObject{{"schemaVersion", 1}, {"entries", QJsonArray{}}}},
                                      {"jobs", QJsonArray{}}, {"installed", QJsonArray{}},
                                      {"backends", QJsonObject{{"flatpak", QJsonObject{{"available", true}}}}}});
    QTRY_COMPARE(changed.count(), 1);
    QCOMPARE(bridge.snapshot().value("catalogue").toMap().value("schemaVersion").toInt(), 1);
    QCOMPARE(bridge.snapshot().value("backends").toMap().value("flatpak").toMap().value("available").toBool(), true);
}

void StoreBridgeTest::rejectsMismatchedReplyWithoutPublishingState() {
    QTemporaryDir directory;
    QLocalServer server;
    const QString path = directory.filePath("store.sock");
    QVERIFY(server.listen(path));
    StoreBridge bridge(path);
    QSignalSpy changed(&bridge, &StoreBridge::snapshotChanged);
    QSignalSpy failed(&bridge, &StoreBridge::errorChanged);
    bridge.refresh();
    QLocalSocket *peer = nullptr;
    const auto request = receiveRequest(server, peer);
    peer->write(QJsonDocument(QJsonObject{{"schemaVersion", 1}, {"requestId", "wrong"}, {"ok", true},
                                      {"result", QJsonObject{{"jobs", QJsonArray{}}}}}).toJson(QJsonDocument::Compact) + '\n');
    QVERIFY(peer->waitForBytesWritten(2000));
    QTRY_VERIFY(failed.count() > 0);
    QCOMPARE(changed.count(), 0);
    QVERIFY(bridge.snapshot().isEmpty());
}

void StoreBridgeTest::enqueuesThenRefreshesRealState() {
    QTemporaryDir directory;
    QLocalServer server;
    const QString path = directory.filePath("store.sock");
    QVERIFY(server.listen(path));
    StoreBridge bridge(path);
    bridge.enqueue(QStringLiteral("sample"), QStringLiteral("install"));
    QLocalSocket *peer = nullptr;
    const auto request = receiveRequest(server, peer);
    QCOMPARE(request.value("operation").toString(), QStringLiteral("enqueue"));
    QCOMPARE(request.value("payload").toObject().value("appId").toString(), QStringLiteral("sample"));
    QCOMPARE(request.value("payload").toObject().value("action").toString(), QStringLiteral("install"));
    answer(peer, request, QJsonObject{{"jobId", "job-1"}});
    QLocalSocket *snapshotPeer = nullptr;
    const auto snapshotRequest = receiveRequest(server, snapshotPeer);
    QCOMPARE(snapshotRequest.value("operation").toString(), QStringLiteral("snapshot"));
}

void StoreBridgeTest::unreachableServiceReportsTranslatableErrorCode() {
    QTemporaryDir directory;
    QVERIFY(directory.isValid());
    StoreBridge bridge(directory.filePath("missing.sock"));
    QSignalSpy failed(&bridge, &StoreBridge::errorChanged);
    bridge.refresh();
    QTRY_VERIFY(failed.count() > 0);
    QCOMPARE(bridge.error(), QStringLiteral("serviceUnavailable"));
}

QTEST_GUILESS_MAIN(StoreBridgeTest)
#include "store_bridge_test.moc"
