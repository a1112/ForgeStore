#include "StoreBridge.h"
#include "WindowGeometry.h"
#include "windoweffects.h"

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
    WindowEffects effects;
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    engine.rootContext()->setContextProperty(QStringLiteral("windowEffects"), &effects);
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreationFailed, &app,
                     []() { QCoreApplication::exit(1); }, Qt::QueuedConnection);
    engine.load(QUrl(QStringLiteral("qrc:/forge-store/qml/Main.qml")));
    if (engine.rootObjects().isEmpty()) return 1;
    auto *window = qobject_cast<QWindow *>(engine.rootObjects().first());
    if (!window) return 1;
#ifdef Q_OS_MACOS
    // AppKit retains native resize/fullscreen behavior behind our visible chrome.
    window->setFlags(Qt::Window);
#else
    window->setFlags(Qt::Window | Qt::FramelessWindowHint);
#endif
    // Allocate the hidden native window before measuring its frame margins.
    window->create();
    effects.setWindow(window);
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
        const auto fitted = fitWindowGeometry(target->availableGeometry(), requested, window->frameMargins(), firstLaunch);
        window->setMinimumSize(fitted.minimum);
        window->setGeometry(fitted.geometry);
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
        if (window->visibility() != QWindow::Windowed) return;
        QList<QRect> screens;
        for (auto *screen : QGuiApplication::screens()) screens.append(screen->geometry());
        const QRect requested = window->geometry();
        if (windowGeometryNeedsRecovery(requested, screens)) fit(target, requested, false);
    });
    QObject::connect(&app, &QCoreApplication::aboutToQuit, &app, [&settings, &normalGeometry]() {
        settings->setValue(QStringLiteral("window/normalGeometry"), normalGeometry);
    });
    bridge.startPolling();
    return app.exec();
}
