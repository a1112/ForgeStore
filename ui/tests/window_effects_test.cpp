#include "StoreBridge.h"
#include "effectspolicy.h"
#include "windoweffects.h"

#include <QGuiApplication>
#include <QAccessible>
#include <QPalette>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickItem>
#include <QQuickWindow>
#include <QTemporaryDir>
#include <QtTest>
#include <memory>

// Synthetic preferences exercise the production QML fallback without changing OS settings.
class SyntheticEffects final : public QObject {
    Q_OBJECT
    Q_PROPERTY(bool enabled MEMBER enabled NOTIFY changed)
    Q_PROPERTY(bool active MEMBER active NOTIFY changed)
    Q_PROPERTY(bool highContrast MEMBER highContrast NOTIFY changed)
    Q_PROPERTY(int cornerRadius MEMBER radius NOTIFY changed)
public:
    bool enabled = true;
    bool active = false;
    bool highContrast = false;
    int radius = 10, titleDoubleClicks = 0;
    Q_INVOKABLE void titlebarDoubleClick() { ++titleDoubleClicks; }
    Q_INVOKABLE void setEnabled(bool value) { enabled = value; emit changed(); }
signals:
    void changed();
};

class SyntheticTitleTarget : public QObject {
    Q_OBJECT
    Q_PROPERTY(int visibility MEMBER visibility NOTIFY changed)
    Q_PROPERTY(QString title MEMBER title CONSTANT)
public:
    int visibility = QWindow::Windowed;
    QString title = QStringLiteral("Synthetic title target");
    int moves = 0, maximizes = 0, restores = 0, minimizes = 0, fullscreens = 0;
    Q_INVOKABLE bool startSystemMove() { ++moves; return true; }
    Q_INVOKABLE void showMaximized() { ++maximizes; }
    Q_INVOKABLE void showNormal() { ++restores; }
    Q_INVOKABLE void showMinimized() { ++minimizes; }
    Q_INVOKABLE void showFullScreen() { ++fullscreens; }
    Q_INVOKABLE void close() {}
signals:
    void changed();
};
class Scene {
public:
    StoreBridge bridge{QStringLiteral("/tmp/unavailable-forge-store-effects.sock")};
    SyntheticEffects effects;
    QQmlEngine engine;
    QQmlComponent component;
    std::unique_ptr<QQuickWindow> window;
    Scene() : component(&engine) {
        engine.rootContext()->setContextProperty("storeBridge", &bridge);
        engine.rootContext()->setContextProperty("windowEffects", &effects);
        component.loadUrl(QUrl(QStringLiteral("qrc:/forge-store/qml/Main.qml")));
        window.reset(qobject_cast<QQuickWindow *>(component.create()));
        if (window) {
            window->setMinimumSize(QSize(1, 1));
            window->show();
        }
    }
    QQuickItem *item(const char *name) {
        return window->findChild<QQuickItem *>(QString::fromLatin1(name));
    }
};

class CloseProbe final : public QObject {
public:
    int requests = 0;
    bool eventFilter(QObject *, QEvent *event) override {
        if (event->type() == QEvent::Close) ++requests;
        return false;
    }
};

class WindowEffectsTest final : public QObject {
    Q_OBJECT
private slots:
    void init() {
        QTest::failOnWarning(QRegularExpression(".*(Binding loop|Only binding to one of multiple|ReferenceError|TypeError).*"));
    }
    void nativeTitleDoubleClickPolicy() {
        using Action = WindowEffectsPolicy::TitlebarAction;
        for (std::string_view preference : {"Minimize", "None", "Maximize", "Zoom", "", "unknown"})
            for (bool fullscreen : {false, true}) for (bool sheet : {false, true})
                for (bool minimize : {false, true}) for (bool zoom : {false, true}) {
                    Action expected = Action::None;
                    if (!fullscreen && !sheet && preference != "None") {
                        if (preference == "Minimize") { if (minimize) expected = Action::Minimize; }
                        else if (zoom) expected = Action::Zoom;
                    }
                    QCOMPARE(WindowEffectsPolicy::titlebarDoubleClickAction(preference,fullscreen,sheet,minimize,zoom),expected);
                }
    }
    void titleGesturesRespectFullscreenModalAndCancel() {
        SyntheticTitleTarget target;
        Scene scene;
        QVERIFY(scene.window);
        auto *chrome=scene.item("windowChrome");
        auto *drag=scene.item("windowDragRegion"); QVERIFY(chrome && drag);
        QVERIFY(chrome->setProperty("targetWindow",QVariant::fromValue(static_cast<QObject *>(&target))));

        scene.window->requestActivate();
        QCoreApplication::processEvents();
        const QPoint origin=drag->mapToScene(QPointF(drag->width()/2,drag->height()/2)).toPoint();
        target.visibility=QWindow::FullScreen; emit target.changed();
        QVERIFY(QMetaObject::invokeMethod(chrome,"titleDoubleClick"));
        QVERIFY(QMetaObject::invokeMethod(chrome,"startTitleDrag"));
        QCOMPARE(scene.effects.titleDoubleClicks,0); QCOMPARE(target.moves,0); QCOMPARE(target.restores,0); QCOMPARE(target.maximizes,0);
        target.visibility=QWindow::Windowed; emit target.changed();
        QTest::mouseDClick(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin);
#ifdef Q_OS_MACOS
        QTRY_COMPARE(scene.effects.titleDoubleClicks,1); QCOMPARE(target.maximizes,0);
#else
        QTRY_COMPARE(target.maximizes,1); QCOMPARE(scene.effects.titleDoubleClicks,0);
#endif
        auto *drawer=scene.window->findChild<QObject *>("detailsDrawer"); QVERIFY(drawer);
        const QVariantMap entry{{"id","synthetic"},{"name","Synthetic"}};
        QVERIFY(QMetaObject::invokeMethod(scene.window.get(),"openDetails",Q_ARG(QVariant,QVariant(entry))));
        QTRY_VERIFY(drawer->property("opened").toBool());
        QVERIFY(!chrome->property("titleGesturesEnabled").toBool());
        const int beforeDouble=scene.effects.titleDoubleClicks, beforeMax=target.maximizes;
        QVERIFY(QMetaObject::invokeMethod(chrome,"titleDoubleClick"));
        QVERIFY(QMetaObject::invokeMethod(chrome,"startTitleDrag"));
        QCOMPARE(scene.effects.titleDoubleClicks,beforeDouble); QCOMPARE(target.maximizes,beforeMax); QCOMPARE(target.moves,0);
        QVERIFY(QMetaObject::invokeMethod(drawer,"close"));
        QTRY_VERIFY(!drawer->property("visible").toBool());
        QVERIFY(chrome->property("titleGesturesEnabled").toBool());
        QTest::mousePress(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin);
        QTest::mouseMove(scene.window.get(),origin+QPoint(40,0),1);
        QTest::mouseMove(scene.window.get(),origin+QPoint(60,0),1);
        QTest::mouseRelease(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin+QPoint(60,0));
        QTRY_COMPARE(target.moves,1);
        QTest::mouseMove(scene.window.get(),origin+QPoint(80,0),1); QCOMPARE(target.moves,1);
        target.visibility=QWindow::FullScreen; emit target.changed();
        QTest::mouseDClick(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin);
        QTest::mousePress(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin);
        QTest::mouseMove(scene.window.get(),origin+QPoint(40,0),1);
        QTest::mouseRelease(scene.window.get(),Qt::LeftButton,Qt::NoModifier,origin+QPoint(40,0));
        QCOMPARE(scene.effects.titleDoubleClicks,beforeDouble); QCOMPARE(target.moves,1); QCOMPARE(target.restores,0);
        auto *button=scene.item("windowMaximizeButton"); QVERIFY(button);
        QVERIFY(QMetaObject::invokeMethod(button,"clicked"));
        QCOMPARE(target.restores,1); QCOMPARE(scene.effects.titleDoubleClicks,beforeDouble);

    }
    void accessibilityPolicyAndPersistedOff() {
        for (bool enabled : {false, true})
            for (bool reduced : {false, true})
                for (bool contrast : {false, true})
                    QCOMPARE(WindowEffectsPolicy::materialAllowed(enabled, reduced, contrast), enabled && !reduced && !contrast);
        QCOMPARE(WindowEffectsPolicy::cornerRadius(false, false), 10);
        QCOMPARE(WindowEffectsPolicy::cornerRadius(true, false), 0);
        QCOMPARE(WindowEffectsPolicy::cornerRadius(false, true), 0);
        QQuickWindow window;
        WindowEffects effects;
        effects.setWindow(&window);
        effects.setEnabled(false);
        QVERIFY(!effects.active());
        WindowEffects reopened;
        QVERIFY(!reopened.enabled());
        reopened.setWindow(&window);
        QVERIFY(!reopened.active());
        reopened.setEnabled(true);
    }
    void geometryEventsDoNotRebuildEffects() {
        QQuickWindow window;
        WindowEffects effects;
        effects.setWindow(&window);
        QSignalSpy changes(&effects, &WindowEffects::changed);
        for (int i = 0; i < 100; ++i) {
            window.resize(400 + i, 300 + i);
            window.setPosition(i, i);
        }
        QCoreApplication::processEvents();
        QCOMPARE(changes.count(), 0);
        window.setWindowState(Qt::WindowMaximized);
        QCOMPARE(effects.cornerRadius(), 0);
        window.setWindowState(Qt::WindowFullScreen);
        QCOMPARE(effects.cornerRadius(), 0);
        window.setWindowState(Qt::WindowNoState);
        QCOMPARE(effects.cornerRadius(), 10);
    }
    void chromeStaysAccessibleAndSeparateFromDragAtAllSizes() {
        Scene scene;
        QVERIFY2(scene.window, qPrintable(scene.component.errorString()));
        auto *drag = scene.item("windowDragRegion");
        auto *handles = scene.item("windowResizeHandles");
        QVERIFY(drag && handles);
        for (const QSize size : {QSize(640, 400), QSize(1040, 700), QSize(1200, 900)}) {
            scene.window->resize(size);
            QCoreApplication::processEvents();
            QCOMPARE(handles->size(), QSizeF(size));
            QCOMPARE(handles->mapToScene(QPointF()), QPointF());
            const QRectF dragRect(drag->mapToScene(QPointF()), drag->size());
            QList<QPair<QString, QRectF>> controlRects;
            for (const char *name : {"windowEffectsButton", "windowMinimizeButton", "windowMaximizeButton", "windowCloseButton"}) {
                auto *button = scene.item(name);
                QVERIFY(button);
                QVERIFY(button->property("focusPolicy").toInt() & Qt::TabFocus);
                auto *accessible = QAccessible::queryAccessibleInterface(button);
                QVERIFY(accessible);
                QVERIFY(!accessible->text(QAccessible::Name).isEmpty());
                const QRectF buttonRect(button->mapToScene(QPointF()), button->size());
                QVERIFY(QRectF(QPointF(), size).contains(buttonRect));
                QVERIFY(!dragRect.intersects(buttonRect));
                controlRects.append({QString::fromLatin1(name), buttonRect});
            }
            int count = 0;
            for (auto *handle : handles->childItems()) {
                if (!handle->property("modelData").isValid()) continue;
                ++count;
                QVERIFY(QRectF(QPointF(), size).contains(QRectF(handle->position(), handle->size())));
                // Geometry is exercised even on macOS, where the native edges replace these handles.
                const QRectF edgeRect(handle->mapToScene(QPointF()), handle->size());
                for (const auto &[name, controlRect] : controlRects) {
                    QVERIFY2(!edgeRect.intersects(controlRect),
                        qPrintable(QStringLiteral("Resize edge %1 overlaps %2 at %3x%4")
                            .arg(handle->property("modelData").toInt()).arg(name).arg(size.width()).arg(size.height())));
                }
            }
            QCOMPARE(count, 8);
        }
    }
    void highContrastAndDisabledMaterialUseOpaqueSurface() {
        Scene scene;
        QVERIFY2(scene.window, qPrintable(scene.component.errorString()));
        auto *surface = scene.item("windowSurface");
        auto *label = scene.item("storeBrandLabel");
        QVERIFY(surface && label);
        QCOMPARE(surface->property("color").value<QColor>().alpha(), 255);
        scene.effects.active = true;
        emit scene.effects.changed();
        QTRY_VERIFY(surface->property("color").value<QColor>().alpha() < 255);
        scene.effects.highContrast = true;
        emit scene.effects.changed();
        QTRY_COMPARE(surface->property("color").value<QColor>(), QGuiApplication::palette().color(QPalette::Window));
        QCOMPARE(label->property("color").value<QColor>(), QGuiApplication::palette().color(QPalette::WindowText));
        scene.effects.active = false;
        scene.effects.highContrast = false;
        scene.effects.radius = 0;
        emit scene.effects.changed();
        QTRY_COMPARE(surface->property("color").value<QColor>(), QColor("#0b101a"));
        QCOMPARE(surface->property("radius").toInt(), 0);
        QCOMPARE(label->property("color").value<QColor>(), QColor("#f2f6fc"));
    }
    void keyboardControlsCloseNormallyAndRestoreFullscreen() {
        Scene scene;
        QVERIFY2(scene.window, qPrintable(scene.component.errorString()));
        scene.window->requestActivate();
        auto *effects = scene.item("windowEffectsButton");
        auto *maximize = scene.item("windowMaximizeButton");
        auto *close = scene.item("windowCloseButton");
        QVERIFY(effects && maximize && close);
        effects->forceActiveFocus();
        QTest::keyClick(scene.window.get(), Qt::Key_Space);
        QVERIFY(!scene.effects.enabled);
        QTest::keySequence(scene.window.get(), QKeySequence(QKeySequence::FullScreen));
        QTRY_COMPARE(scene.window->visibility(), QWindow::FullScreen);
        maximize->forceActiveFocus();
        QTest::keyClick(scene.window.get(), Qt::Key_Space);
        QTRY_COMPARE(scene.window->visibility(), QWindow::Windowed);
        CloseProbe closed;
        scene.window->installEventFilter(&closed);
        close->forceActiveFocus();
        QTest::keyClick(scene.window.get(), Qt::Key_Space);
        QTRY_COMPARE(closed.requests, 1);
        QTRY_VERIFY(!scene.window->isVisible());
        scene.window->show();
        scene.window->requestActivate();
        QCoreApplication::processEvents();
        QTest::keySequence(scene.window.get(), QKeySequence(QKeySequence::Close));
        QTRY_COMPARE(closed.requests, 2);
        QTRY_VERIFY(!scene.window->isVisible());
    }
    void modalDetailsKeepWindowControlsAndResizeOutOfTheGesture() {
        Scene scene;
        QVERIFY2(scene.window, qPrintable(scene.component.errorString()));
        scene.window->requestActivate();
        auto *drawer = scene.window->findChild<QObject *>("detailsDrawer");
        auto *handles = scene.item("windowResizeHandles");
        auto *close = scene.item("windowCloseButton");
        QVERIFY(drawer && handles && close);
        const QVariantMap entry{{"id", "synthetic"}, {"name", "Synthetic"}};
        QVERIFY(QMetaObject::invokeMethod(scene.window.get(), "openDetails", Q_ARG(QVariant, QVariant(entry))));
        QTRY_VERIFY(drawer->property("opened").toBool());
        QVERIFY(!handles->isEnabled());
        // Clicking the dimmed title bar is an outside-popup gesture, never an app close.
        QTest::mouseClick(scene.window.get(), Qt::LeftButton, Qt::NoModifier,
                          close->mapToScene(QPointF(close->width() / 2, close->height() / 2)).toPoint());
        QVERIFY(scene.window->isVisible());
        QTest::keyClick(scene.window.get(), Qt::Key_Escape);
        QTRY_VERIFY(!drawer->property("visible").toBool());
        QVERIFY(scene.window->isVisible());
    }
};

int main(int argc, char **argv) {
    qputenv("QT_QPA_PLATFORM", "offscreen");
    qputenv("QT_QUICK_BACKEND", "software");
    QTemporaryDir profile;
    if (!profile.isValid()) return 2;
    qputenv("FORGE_STORE_WINDOW_SETTINGS_PATH", (profile.path() + "/settings.ini").toUtf8());
    QGuiApplication app(argc, argv);
    app.setQuitOnLastWindowClosed(false);
    WindowEffectsTest test;
    return QTest::qExec(&test, argc, argv);
}
#include "window_effects_test.moc"
