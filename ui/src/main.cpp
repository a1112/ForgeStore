#include "StoreBridge.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>
#include <QQuickWindow>
#include <QScreen>
#include <QSettings>
#include <algorithm>
#include <memory>

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
    auto *window = qobject_cast<QWindow *>(engine.rootObjects().first());
    if (!window) return 1;
    const QString settingsPath = qEnvironmentVariable("FORGE_STORE_WINDOW_SETTINGS_PATH");
    std::unique_ptr<QSettings> settings = settingsPath.isEmpty()
        ? std::make_unique<QSettings>(QStringLiteral("Forge"), QStringLiteral("ForgeStore"))
        : std::make_unique<QSettings>(settingsPath, QSettings::IniFormat);
    const QRect saved = settings->value(QStringLiteral("window/normalGeometry")).toRect();
    QScreen *screen = saved.isValid() ? QGuiApplication::screenAt(saved.center()) : nullptr;
    if (!screen) screen = window->screen();
    if (!screen) screen = QGuiApplication::primaryScreen();
    auto fit = [window](QScreen *target, QRect requested, bool firstLaunch) {
        if (!target) return;
        // Qt screen and window geometries are already device independent.
        const QMargins frame = window->frameMargins();
        const int edge = firstLaunch ? 16 : 0;
        const QRect area = target->availableGeometry().adjusted(edge + frame.left(), edge + frame.top(), -edge - frame.right(), -edge - frame.bottom());
        if (area.isEmpty()) return;
        window->setMinimumSize(QSize(std::min(820, area.width()), std::min(580, area.height())));
        const int capWidth = firstLaunch ? int(area.width() * 0.84) : area.width();
        const int capHeight = firstLaunch ? int(area.height() * 0.84) : area.height();
        requested.setSize(QSize(std::clamp(requested.width(), window->minimumWidth(), std::max(window->minimumWidth(), capWidth)),
                                std::clamp(requested.height(), window->minimumHeight(), std::max(window->minimumHeight(), capHeight))));
        if (firstLaunch) requested.moveCenter(area.center());
        requested.moveLeft(std::clamp(requested.x(), area.left(), area.right() - requested.width() + 1));
        requested.moveTop(std::clamp(requested.y(), area.top(), area.bottom() - requested.height() + 1));
        window->setGeometry(requested);
    };
    fit(screen, saved.isValid() ? saved : QRect(0, 0, 1040, 700), !saved.isValid());
    window->show();
    QRect normalGeometry = window->geometry();
    auto remember = [window, &normalGeometry]() {
        if (window->visibility() == QWindow::Windowed) normalGeometry = window->geometry();
    };
    QObject::connect(window, &QWindow::xChanged, &app, remember);
    QObject::connect(window, &QWindow::yChanged, &app, remember);
    QObject::connect(window, &QWindow::widthChanged, &app, remember);
    QObject::connect(window, &QWindow::heightChanged, &app, remember);
    QObject::connect(window, &QWindow::screenChanged, &app, [window, fit](QScreen *target) {
        if (window->visibility() == QWindow::Windowed) fit(target, window->geometry(), false);
    });
    QObject::connect(&app, &QCoreApplication::aboutToQuit, &app, [&settings, &normalGeometry]() {
        settings->setValue(QStringLiteral("window/normalGeometry"), normalGeometry);
    });
    bridge.startPolling();
    return app.exec();
}
