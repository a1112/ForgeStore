#include "StoreBridge.h"

#include <QGuiApplication>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QProcess>
#include <QProcessEnvironment>
#include <QQuickItem>
#include <QtTest>

#include <memory>

class StoreWindowTest final : public QObject {
    Q_OBJECT

private slots:
    void loadsChineseDesktopAndCanSwitchToEnglish();
    void loadsBundledQmlResources();
    void launchedApplicationKeepsWindowAlive();
    void treatsCatalogueNamesAsPlainText();
};

void StoreWindowTest::loadsChineseDesktopAndCanSwitchToEnglish() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    QVERIFY2(component.isReady(), qPrintable(component.errorString()));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    QCOMPARE(window->objectName(), QStringLiteral("forgeStoreWindow"));
    QCOMPARE(window->property("language").toString(), QStringLiteral("zh_CN"));
    QCOMPARE(window->property("title").toString(), QStringLiteral("Forge 应用市场"));
    QVERIFY(window->setProperty("language", QStringLiteral("en")));
    QTRY_COMPARE(window->property("title").toString(), QStringLiteral("Forge Store"));
}

void StoreWindowTest::loadsBundledQmlResources() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl(QStringLiteral("qrc:/forge-store/qml/Main.qml")));
    QVERIFY2(component.isReady(), qPrintable(component.errorString()));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    QCOMPARE(window->property("title").toString(), QStringLiteral("Forge 应用市场"));
}

void StoreWindowTest::launchedApplicationKeepsWindowAlive() {
    QProcess process;
    auto environment = QProcessEnvironment::systemEnvironment();
    environment.insert(QStringLiteral("QT_QPA_PLATFORM"), QStringLiteral("offscreen"));
    environment.insert(QStringLiteral("FORGE_STORE_SOFTWARE_RENDER"), QStringLiteral("1"));
    environment.insert(QStringLiteral("XDG_RUNTIME_DIR"), QStringLiteral("/tmp"));
    process.setProcessEnvironment(environment);
    process.start(QStringLiteral(FORGE_STORE_UI_BINARY));
    QVERIFY2(process.waitForStarted(2000), qPrintable(process.errorString()));
    QTest::qWait(300);
    QVERIFY2(process.state() == QProcess::Running,
             qPrintable(QStringLiteral("Store exited: %1").arg(QString::fromUtf8(process.readAllStandardError()))));
    process.kill();
    QVERIFY(process.waitForFinished(2000));
}

void StoreWindowTest::treatsCatalogueNamesAsPlainText() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    const QVariantMap entry{{"id", "probe"}, {"name", QVariantMap{{"zhCN", "<b>测试</b>"}, {"en", "Probe"}}},
                            {"summary", QVariantMap{{"zhCN", "摘要"}, {"en", "Summary"}}},
                            {"version", "1"}, {"license", "MIT"},
                            {"delivery", QVariantMap{{"backend", "flatpak"}}}};
    const QVariantMap state{{"catalogue", QVariantMap{{"schemaVersion", 1}, {"entries", QVariantList{entry}}}},
                            {"jobs", QVariantList{}}, {"installed", QVariantList{}},
                            {"backends", QVariantMap{{"flatpak", QVariantMap{{"available", true}}}}}};
    QVERIFY(window->setProperty("state", state));
    QTRY_COMPARE(window->property("entries").toList().size(), 1);
    QTRY_COMPARE(window->property("visibleEntries").toList().size(), 1);
    auto *repeater = window->findChild<QObject *>("discoverRepeater");
    QVERIFY(repeater != nullptr);
    QCOMPARE(repeater->property("count").toInt(), 1);
    QQuickItem *card = nullptr;
    QVERIFY(QMetaObject::invokeMethod(repeater, "itemAt", Q_RETURN_ARG(QQuickItem *, card), Q_ARG(int, 0)));
    QVERIFY(card != nullptr);
    auto *name = card->findChild<QObject *>("appName");
    QVERIFY(name != nullptr);
    QCOMPARE(name->property("text").toString(), QStringLiteral("<b>测试</b>"));
    QCOMPARE(name->property("textFormat").toInt(), 0); // Text.PlainText
}

int main(int argc, char **argv) {
    QGuiApplication app(argc, argv);
    StoreWindowTest test;
    return QTest::qExec(&test, argc, argv);
}

#include "qml_smoke_test.moc"
