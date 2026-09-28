#include "StoreBridge.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>
#include <QQuickWindow>

int main(int argc, char **argv) {
    QQuickStyle::setStyle(QStringLiteral("Basic"));
    if (qEnvironmentVariable("FORGE_STORE_SOFTWARE_RENDER") == QStringLiteral("1")) {
        QQuickWindow::setGraphicsApi(QSGRendererInterface::Software);
    }
    QGuiApplication app(argc, argv);
    app.setApplicationName(QStringLiteral("Forge Store"));
    app.setDesktopFileName(QStringLiteral("forge-store"));

    const QString runtimeDirectory = qEnvironmentVariable("XDG_RUNTIME_DIR");
    StoreBridge bridge(runtimeDirectory + QStringLiteral("/forge-store/store.sock"));
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreationFailed, &app,
                     []() { QCoreApplication::exit(1); }, Qt::QueuedConnection);
    engine.load(QUrl(QStringLiteral("qrc:/forge-store/qml/Main.qml")));
    if (engine.rootObjects().isEmpty()) return 1;
    bridge.startPolling();
    return app.exec();
}
