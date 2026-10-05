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
    void searchesCatalogueIds();
    void cardsFillTheirPagesAtReferenceSize();
    void actionsRespectBackendAndJobState();
    void unavailableBackendsDisableMutationsAndShowReasons();
    void longMetadataKeepsActionsInsideCard();
    void nativeBackendsHaveTranslatedLabelsAndNoUnsafeCancel();
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
    QVERIFY(window->findChild<QObject *>(QStringLiteral("detailsDrawer")) != nullptr);
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

void StoreWindowTest::searchesCatalogueIds() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    const QVariantMap entry{{"id", "7zip"}, {"name", QVariantMap{{"zhCN", "压缩工具"}, {"en", "Archive tool"}}},
                            {"summary", QVariantMap{{"zhCN", "文件管理"}, {"en", "File manager"}}}};
    QVERIFY(window->setProperty("state", QVariantMap{{"catalogue", QVariantMap{{"entries", QVariantList{entry}}}}}));
    QVERIFY(window->setProperty("query", QStringLiteral("7zip")));
    QTRY_COMPARE(window->property("visibleEntries").toList().size(), 1);
}

void StoreWindowTest::cardsFillTheirPagesAtReferenceSize() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    QCOMPARE(window->property("width").toInt(), 1160);
    QCOMPARE(window->property("height").toInt(), 760);
    const QVariantMap entry{{"id", "7zip"}, {"name", QVariantMap{{"zhCN", "压缩工具"}}},
                            {"delivery", QVariantMap{{"backend", "compatforge"}}}};
    const QVariantMap job{{"id", "job-1"}, {"appId", "7zip"}, {"backend", "compatforge"},
                          {"action", "install"}, {"state", "failed"}, {"detail", "backend unavailable"}};
    const QVariantMap installed{{"appId", "7zip"}, {"backend", "compatforge"},
                                {"version", "1"}, {"canRollback", false}};
    const QVariantMap backends{{"compatforge", QVariantMap{{"available", false}, {"reason", "service unavailable"}}}};
    const QVariantMap state{{"catalogue", QVariantMap{{"entries", QVariantList{entry}}}},
                            {"jobs", QVariantList{job}}, {"installed", QVariantList{installed}},
                            {"backends", backends}};
    QVERIFY(window->setProperty("state", state));

    for (const QString &page : {QStringLiteral("discover"), QStringLiteral("installed"),
                                QStringLiteral("queue"), QStringLiteral("diagnostics")}) {
        QVERIFY(window->setProperty("page", page));
        auto *repeater = window->findChild<QObject *>(page + QStringLiteral("Repeater"));
        QVERIFY2(repeater != nullptr, qPrintable(page + QStringLiteral(" repeater missing")));
        QQuickItem *card = nullptr;
        QVERIFY(QMetaObject::invokeMethod(repeater, "itemAt", Q_RETURN_ARG(QQuickItem *, card), Q_ARG(int, 0)));
        QVERIFY(card != nullptr);
        QTRY_VERIFY2(card->width() >= 600, qPrintable(page + QStringLiteral(" card too narrow: ") + QString::number(card->width())));
    }
}

void StoreWindowTest::actionsRespectBackendAndJobState() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));

    const QVariantMap packageEntry{{"id", "forge-app"}, {"name", QVariantMap{{"zhCN", "Forge 应用"}}},
                                   {"delivery", QVariantMap{{"backend", "forge-package"}}}};
    const QVariantMap compatEntry{{"id", "compat-app"}, {"name", QVariantMap{{"zhCN", "兼容应用"}}},
                                  {"delivery", QVariantMap{{"backend", "compatforge"}}}};
    const QVariantMap packageInstall{{"appId", "forge-app"}, {"backend", "forge-package"},
                                     {"version", "1"}, {"canRollback", true}};
    const QVariantMap compatInstall{{"appId", "compat-app"}, {"backend", "compatforge"},
                                    {"version", "1"}, {"canRollback", false}};
    const QVariantMap runningPackage{{"id", "job-package"}, {"appId", "forge-app"},
                                     {"backend", "forge-package"}, {"action", "update"}, {"state", "running"}};
    const QVariantMap interruptedCompat{{"id", "job-interrupted"}, {"appId", "compat-app"},
                                        {"backend", "compatforge"}, {"action", "install"}, {"state", "interrupted"}};
    const QVariantMap queuedPackage{{"id", "job-queued"}, {"appId", "forge-app"},
                                    {"backend", "forge-package"}, {"action", "update"}, {"state", "queued"}};
    const QVariantMap runningCompat{{"id", "job-compat"}, {"appId", "compat-app"},
                                    {"backend", "compatforge"}, {"action", "install"}, {"state", "running"}};
    const QVariantMap state{{"catalogue", QVariantMap{{"entries", QVariantList{packageEntry, compatEntry}}}},
                            {"installed", QVariantList{packageInstall, compatInstall}},
                            {"jobs", QVariantList{runningPackage, interruptedCompat, queuedPackage, runningCompat}},
                            {"backends", QVariantMap{}}};
    QVERIFY(window->setProperty("state", state));
    QVERIFY(window->setProperty("page", QStringLiteral("installed")));
    auto *installedRepeater = window->findChild<QObject *>(QStringLiteral("installedRepeater"));
    QVERIFY(installedRepeater != nullptr);
    QQuickItem *packageCard = nullptr;
    QQuickItem *compatCard = nullptr;
    QVERIFY(QMetaObject::invokeMethod(installedRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, packageCard), Q_ARG(int, 0)));
    QVERIFY(QMetaObject::invokeMethod(installedRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, compatCard), Q_ARG(int, 1)));
    QVERIFY(packageCard != nullptr && compatCard != nullptr);
    auto *packageUninstall = packageCard->findChild<QObject *>(QStringLiteral("installedUninstallButton"));
    auto *compatUninstall = compatCard->findChild<QObject *>(QStringLiteral("installedUninstallButton"));
    QVERIFY(packageUninstall != nullptr && compatUninstall != nullptr);
    QCOMPARE(packageUninstall->property("visible").toBool(), false);
    QCOMPARE(compatUninstall->property("visible").toBool(), true);

    QVERIFY(QMetaObject::invokeMethod(window.get(), "openDetails", Q_ARG(QVariant, QVariant(packageEntry))));
    auto *drawer = window->findChild<QObject *>(QStringLiteral("detailsDrawer"));
    QVERIFY(drawer != nullptr);
    QTRY_VERIFY(drawer->property("opened").toBool());
    auto *detailsUninstall = drawer->findChild<QObject *>(QStringLiteral("detailsUninstallButton"));
    QVERIFY(detailsUninstall != nullptr);
    QCOMPARE(detailsUninstall->property("visible").toBool(), false);
    QVERIFY(window->setProperty("selectedEntry", compatEntry));
    QTRY_COMPARE(detailsUninstall->property("visible").toBool(), true);

    QVERIFY(window->setProperty("page", QStringLiteral("queue")));
    auto *queueRepeater = window->findChild<QObject *>(QStringLiteral("queueRepeater"));
    QVERIFY(queueRepeater != nullptr);
    QQuickItem *runningPackageCard = nullptr;
    QQuickItem *interruptedCard = nullptr;
    QQuickItem *queuedPackageCard = nullptr;
    QQuickItem *runningCompatCard = nullptr;
    QVERIFY(QMetaObject::invokeMethod(queueRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, runningPackageCard), Q_ARG(int, 0)));
    QVERIFY(QMetaObject::invokeMethod(queueRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, interruptedCard), Q_ARG(int, 1)));
    QVERIFY(QMetaObject::invokeMethod(queueRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, queuedPackageCard), Q_ARG(int, 2)));
    QVERIFY(QMetaObject::invokeMethod(queueRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, runningCompatCard), Q_ARG(int, 3)));
    QVERIFY(runningPackageCard && interruptedCard && queuedPackageCard && runningCompatCard);
    auto *packageCancel = runningPackageCard->findChild<QObject *>(QStringLiteral("queueCancelButton"));
    auto *interruptedRetry = interruptedCard->findChild<QObject *>(QStringLiteral("queueRetryButton"));
    auto *queuedCancel = queuedPackageCard->findChild<QObject *>(QStringLiteral("queueCancelButton"));
    auto *compatCancel = runningCompatCard->findChild<QObject *>(QStringLiteral("queueCancelButton"));
    QVERIFY(packageCancel && interruptedRetry && queuedCancel && compatCancel);
    QCOMPARE(packageCancel->property("visible").toBool(), false);
    QCOMPARE(interruptedRetry->property("enabled").toBool(), true);
    QCOMPARE(queuedCancel->property("visible").toBool(), true);
    QCOMPARE(compatCancel->property("visible").toBool(), true);
}

void StoreWindowTest::unavailableBackendsDisableMutationsAndShowReasons() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));

    const QVariantMap compatEntry{{"id", "compat-app"}, {"name", QVariantMap{{"zhCN", "兼容应用"}}},
                                  {"delivery", QVariantMap{{"backend", "compatforge"}}}};
    const QVariantMap packageEntry{{"id", "forge-app"}, {"name", QVariantMap{{"zhCN", "Forge 应用"}}},
                                   {"delivery", QVariantMap{{"backend", "forge-package"}}}};
    const QVariantMap compatInstall{{"appId", "compat-app"}, {"backend", "compatforge"},
                                    {"version", "1"}, {"canRollback", true}};
    const QVariantMap packageInstall{{"appId", "forge-app"}, {"backend", "forge-package"},
                                     {"version", "1"}, {"canRollback", true}};
    const QVariantMap unavailable{{"compatforge", QVariantMap{{"available", false}, {"reason", "CompatForge offline"}}},
                                  {"forgePackage", QVariantMap{{"available", false}, {"reason", "Package service offline"}}}};
    const QVariantMap catalogue{{"entries", QVariantList{compatEntry, packageEntry}}};
    const QVariantList installed{compatInstall, packageInstall};
    QVERIFY(window->setProperty("state", QVariantMap{{"catalogue", catalogue}, {"installed", installed},
                                                    {"jobs", QVariantList{}}, {"backends", unavailable}}));
    QVERIFY(window->setProperty("page", QStringLiteral("installed")));
    auto *installedRepeater = window->findChild<QObject *>(QStringLiteral("installedRepeater"));
    QVERIFY(installedRepeater != nullptr);
    QQuickItem *compatCard = nullptr;
    QQuickItem *packageCard = nullptr;
    QVERIFY(QMetaObject::invokeMethod(installedRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, compatCard), Q_ARG(int, 0)));
    QVERIFY(QMetaObject::invokeMethod(installedRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, packageCard), Q_ARG(int, 1)));
    QVERIFY(compatCard && packageCard);
    auto *uninstall = compatCard->findChild<QObject *>(QStringLiteral("installedUninstallButton"));
    QVERIFY(uninstall != nullptr);
    QCOMPARE(uninstall->property("enabled").toBool(), false);
    auto *update = compatCard->findChild<QObject *>(QStringLiteral("installedUpdateButton"));
    auto *rollback = compatCard->findChild<QObject *>(QStringLiteral("installedRollbackButton"));
    auto *packageRollback = packageCard->findChild<QObject *>(QStringLiteral("installedRollbackButton"));
    auto *installedReason = compatCard->findChild<QObject *>(QStringLiteral("installedBackendReason"));
    QVERIFY(update && rollback && packageRollback && installedReason);
    QCOMPARE(update->property("enabled").toBool(), false);
    QCOMPARE(rollback->property("enabled").toBool(), false);
    QCOMPARE(packageRollback->property("enabled").toBool(), false);
    QCOMPARE(installedReason->property("text").toString(), QStringLiteral("CompatForge offline"));

    QVERIFY(window->setProperty("page", QStringLiteral("discover")));
    auto *discoverRepeater = window->findChild<QObject *>(QStringLiteral("discoverRepeater"));
    QVERIFY(discoverRepeater != nullptr);
    QQuickItem *discoverCard = nullptr;
    QVERIFY(QMetaObject::invokeMethod(discoverRepeater, "itemAt", Q_RETURN_ARG(QQuickItem *, discoverCard), Q_ARG(int, 0)));
    QVERIFY(discoverCard != nullptr);
    auto *discoverInstall = discoverCard->findChild<QObject *>(QStringLiteral("discoverInstallButton"));
    auto *discoverReason = discoverCard->findChild<QObject *>(QStringLiteral("discoverBackendReason"));
    QVERIFY(discoverInstall && discoverReason);
    QCOMPARE(discoverInstall->property("enabled").toBool(), false);
    QCOMPARE(discoverReason->property("text").toString(), QStringLiteral("CompatForge offline"));

    QVERIFY(QMetaObject::invokeMethod(window.get(), "openDetails", Q_ARG(QVariant, QVariant(compatEntry))));
    auto *drawer = window->findChild<QObject *>(QStringLiteral("detailsDrawer"));
    QVERIFY(drawer != nullptr);
    auto *detailsInstall = drawer->findChild<QObject *>(QStringLiteral("detailsInstallButton"));
    auto *detailsUninstall = drawer->findChild<QObject *>(QStringLiteral("detailsUninstallButton"));
    QVERIFY(detailsInstall && detailsUninstall);
    QCOMPARE(detailsInstall->property("enabled").toBool(), false);
    QCOMPARE(detailsUninstall->property("enabled").toBool(), false);

    const QVariantMap available{{"compatforge", QVariantMap{{"available", true}}},
                                {"forgePackage", QVariantMap{{"available", true}}}};
    QVERIFY(window->setProperty("state", QVariantMap{{"catalogue", catalogue}, {"installed", installed},
                                                    {"jobs", QVariantList{}}, {"backends", available}}));
    QTRY_COMPARE(uninstall->property("enabled").toBool(), true);
    QCOMPARE(update->property("enabled").toBool(), true);
    QCOMPARE(rollback->property("enabled").toBool(), true);
    QCOMPARE(packageRollback->property("enabled").toBool(), true);
    QCOMPARE(discoverInstall->property("enabled").toBool(), true);
    QCOMPARE(detailsInstall->property("enabled").toBool(), true);
    QCOMPARE(detailsUninstall->property("enabled").toBool(), true);
}

void StoreWindowTest::longMetadataKeepsActionsInsideCard() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    const QVariantMap entry{{"id", "7zip"}, {"name", QVariantMap{{"zhCN", "7-Zip"}}},
                            {"summary", QVariantMap{{"zhCN", "压缩与解压文件"}}},
                            {"version", "26.01"},
                            {"license", "LGPL-2.1-or-later AND BSD-3-Clause AND BSD-2-Clause AND LicenseRef-unRAR-restriction"},
                            {"delivery", QVariantMap{{"backend", "compatforge"}}}};
    const QVariantMap state{{"catalogue", QVariantMap{{"entries", QVariantList{entry}}}},
                            {"jobs", QVariantList{}}, {"installed", QVariantList{}},
                            {"backends", QVariantMap{{"compatforge", QVariantMap{{"available", true}}}}}};
    QVERIFY(window->setProperty("state", state));
    auto *repeater = window->findChild<QObject *>(QStringLiteral("discoverRepeater"));
    QVERIFY(repeater != nullptr);
    QQuickItem *card = nullptr;
    QVERIFY(QMetaObject::invokeMethod(repeater, "itemAt", Q_RETURN_ARG(QQuickItem *, card), Q_ARG(int, 0)));
    QVERIFY(card != nullptr);
    QQuickItem *detailsButton = nullptr;
    for (auto *child : card->findChildren<QObject *>()) {
        if (QString::fromLatin1(child->metaObject()->className()).contains(QStringLiteral("Button"))
            && child->property("text").toString() == QStringLiteral("详情")) {
            detailsButton = qobject_cast<QQuickItem *>(child);
            break;
        }
    }
    auto *installButton = card->findChild<QQuickItem *>(QStringLiteral("discoverInstallButton"));
    QVERIFY(detailsButton != nullptr && installButton != nullptr);

    for (const QSize size : {QSize(1160, 760), QSize(820, 580)}) {
        QVERIFY(window->setProperty("width", size.width()));
        QVERIFY(window->setProperty("height", size.height()));
        QTRY_VERIFY(card->width() > 400);
        QTest::qWait(80);
        for (QQuickItem *button : {detailsButton, installButton}) {
            const QPointF position = button->mapToItem(card, QPointF(0, 0));
            QVERIFY2(button->width() >= 80, "action button shrank below a useful width");
            QVERIFY2(position.x() >= -1 && position.x() + button->width() <= card->width() + 1,
                     qPrintable(QStringLiteral("%1 button clipped at %2x%3: x=%4 width=%5 card=%6")
                                    .arg(button->property("text").toString())
                                    .arg(size.width()).arg(size.height())
                                    .arg(position.x()).arg(button->width()).arg(card->width())));
        }
    }
}

void StoreWindowTest::nativeBackendsHaveTranslatedLabelsAndNoUnsafeCancel() {
    StoreBridge bridge(QStringLiteral("/tmp/unavailable-store.sock"));
    QQmlEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("storeBridge"), &bridge);
    QQmlComponent component(&engine, QUrl::fromLocalFile(QStringLiteral(FORGE_STORE_UI_SOURCE_DIR "/qml/Main.qml")));
    std::unique_ptr<QObject> window(component.create());
    QVERIFY2(window != nullptr, qPrintable(component.errorString()));
    QVariant label;
    QVERIFY(QMetaObject::invokeMethod(window.get(), "backendLabel", Q_RETURN_ARG(QVariant,label), Q_ARG(QVariant,QStringLiteral("ubuntu-deb"))));
    QCOMPARE(label.toString(), QStringLiteral("Ubuntu 软件包"));
    QVERIFY(QMetaObject::invokeMethod(window.get(), "backendLabel", Q_RETURN_ARG(QVariant,label), Q_ARG(QVariant,QStringLiteral("snap"))));
    QCOMPARE(label.toString(), QStringLiteral("Snap 应用"));
    QVERIFY(window->setProperty("page", QStringLiteral("queue")));
    const QVariantMap job{{"id","job-1"},{"appId","org.forge.test"},{"backend","ubuntu-deb"},
        {"action","install"},{"state","running"}};
    QVERIFY(window->setProperty("state",QVariantMap{{"jobs",QVariantList{job}}}));
    auto *repeater=window->findChild<QObject *>(QStringLiteral("queueRepeater"));
    QVERIFY(repeater!=nullptr);
    QQuickItem *card=nullptr;
    QVERIFY(QMetaObject::invokeMethod(repeater,"itemAt",Q_RETURN_ARG(QQuickItem *,card),Q_ARG(int,0)));
    QVERIFY(card!=nullptr);
    auto *cancel=card->findChild<QObject *>(QStringLiteral("queueCancelButton"));
    QVERIFY(cancel!=nullptr);
    QCOMPARE(cancel->property("visible").toBool(),false);
    auto recovering=job;
    recovering["state"]=QStringLiteral("queued");recovering["nativePending"]=true;
    QVERIFY(window->setProperty("state",QVariantMap{{"jobs",QVariantList{recovering}}}));
    QVERIFY(QMetaObject::invokeMethod(repeater,"itemAt",Q_RETURN_ARG(QQuickItem *,card),Q_ARG(int,0)));
    QVERIFY(card!=nullptr);
    cancel=card->findChild<QObject *>(QStringLiteral("queueCancelButton"));
    QVERIFY(cancel!=nullptr);
    QCOMPARE(cancel->property("visible").toBool(),false);
}

int main(int argc, char **argv) {
    QGuiApplication app(argc, argv);
    StoreWindowTest test;
    return QTest::qExec(&test, argc, argv);
}

#include "qml_smoke_test.moc"
