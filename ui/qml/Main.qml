import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../translations/Strings.js" as Strings

ApplicationWindow {
    id: root
    component SafeLabel: Label { textFormat: Text.PlainText }
    objectName: "forgeStoreWindow"
    width: 1160
    height: 760
    minimumWidth: 820
    minimumHeight: 580
    visible: true
    color: "#0b101a"
    property string language: "zh_CN"
    property var labels: Strings.forLanguage(language)
    title: labels.title
    property string page: "discover"
    property string query: ""
    property var selectedEntry: null
    property var state: storeBridge.snapshot
    property var entries: state.catalogue && state.catalogue.entries ? state.catalogue.entries : []
    property var jobs: state.jobs || []
    property var installed: state.installed || []
    property var backends: state.backends || ({})
    property var visibleEntries: entries.filter(function(entry) {
        const needle = query.trim().toLocaleLowerCase()
        if (needle.length === 0) return true
        const fields = [entry.id || "", localized(entry.name), localized(entry.summary), entry.publisher || "", entry.origin || ""]
        return fields.some(function(field) { return String(field).toLocaleLowerCase().indexOf(needle) >= 0 })
    })

    function localized(value) {
        if (value === undefined || value === null) return ""
        if (typeof value === "string") return value
        return language === "en" ? (value.en || value.zhCN || "") : (value.zhCN || value.en || "")
    }
    function installedRecord(appId) {
        for (const item of installed) if (item.appId === appId) return item
        return null
    }
    function findEntry(appId) {
        for (const item of entries) if (item.id === appId) return item
        return null
    }
    function backendKey(entry) {
        const backend = entry && entry.delivery ? entry.delivery.backend : ""
        return backend === "forge-package" ? "forgePackage" : backend
    }
    function backendLabel(backend) {
        if (backend === "compatforge") return labels.windows
        if (backend === "flatpak") return labels.flatpak
        if (backend === "forge-package" || backend === "forgePackage") return labels.forgePackage
        return backend || labels.unknown
    }
    function backendReady(entry) {
        const record = backends[backendKey(entry)]
        return record !== undefined && record.available === true
    }
    function backendReason(entry) {
        const record = backends[backendKey(entry)]
        return record && record.reason ? record.reason : labels.unavailable
    }
    function permissionList(entry) {
        const permissions = entry && entry.permissions ? entry.permissions : []
        if (Array.isArray(permissions)) return permissions.map(function(item) { return String(item) }).join(" · ")
        return String(permissions)
    }
    function statusLabel(state) { return labels[state] || state || labels.unknown }
    function displayedError() { return storeBridge.error ? (labels[storeBridge.error] || storeBridge.error) : "" }
    function openDetails(entry) { selectedEntry = entry; details.open() }

    Shortcut { sequence: "Ctrl+F"; onActivated: { root.page = "discover"; searchField.forceActiveFocus() } }
    Shortcut { sequence: "Ctrl+R"; onActivated: storeBridge.refresh() }
    Shortcut { sequence: "Escape"; onActivated: details.close() }

    background: Rectangle { color: "#0b101a" }

    header: Rectangle {
        height: 70
        color: "#142135"
        border.color: "#283b56"
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 24
            anchors.rightMargin: 22
            spacing: 16
            Rectangle {
                width: 36; height: 36; radius: 11
                color: "#3987d8"
                SafeLabel { anchors.centerIn: parent; text: "◆"; color: "white"; font.pixelSize: 20 }
            }
            ColumnLayout {
                spacing: 0
                SafeLabel { text: "Forge"; color: "#f2f6fc"; font.pixelSize: 21; font.bold: true }
                SafeLabel { text: root.labels.title; color: "#a9bed8"; font.pixelSize: 12 }
            }
            Item { Layout.fillWidth: true }
            SafeLabel { text: root.labels.language; color: "#b6c9de" }
            ComboBox {
                id: languageChoice
                Layout.preferredWidth: 142
                model: [root.labels.chinese, root.labels.english]
                currentIndex: root.language === "en" ? 1 : 0
                onActivated: root.language = currentIndex === 1 ? "en" : "zh_CN"
                Accessible.name: root.labels.language
            }
            Button { text: root.labels.refresh; onClicked: storeBridge.refresh(); Accessible.name: root.labels.refresh }
        }
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 16

        Rectangle {
            Layout.preferredWidth: 194
            Layout.fillHeight: true
            radius: 17
            color: "#142135"
            border.color: "#283b56"
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 14
                spacing: 8
                Repeater {
                    model: [
                        {key: "discover", caption: root.labels.discover, icon: "⌕"},
                        {key: "installed", caption: root.labels.installed, icon: "▣"},
                        {key: "queue", caption: root.labels.queue, icon: "≡"},
                        {key: "diagnostics", caption: root.labels.diagnostics, icon: "⚙"}
                    ]
                    delegate: Button {
                        required property var modelData
                        Layout.fillWidth: true
                        text: modelData.icon + "   " + modelData.caption
                        highlighted: root.page === modelData.key
                        onClicked: root.page = modelData.key
                        Accessible.name: modelData.caption
                    }
                }
                Item { Layout.fillHeight: true }
                SafeLabel {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: root.labels.continueInBackground
                    color: "#8da6c2"
                    font.pixelSize: 12
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 17
            color: "#142135"
            border.color: "#283b56"
            clip: true

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 22
                spacing: 14

                RowLayout {
                    Layout.fillWidth: true
                    SafeLabel {
                        text: root.labels[root.page]
                        color: "#f2f6fc"
                        font.pixelSize: 27
                        font.bold: true
                    }
                    Item { Layout.fillWidth: true }
                    SafeLabel { text: root.page === "discover" ? root.visibleEntries.length + " / " + root.entries.length : ""; color: "#8da6c2" }
                }

                TextField {
                    id: searchField
                    Layout.fillWidth: true
                    visible: root.page === "discover"
                    placeholderText: root.labels.search
                    text: root.query
                    onTextChanged: root.query = text
                    Accessible.name: root.labels.search
                }

                SafeLabel {
                    Layout.fillWidth: true
                    visible: root.page === "discover"
                    text: root.labels.searchHint
                    color: "#9bb1cb"
                }

                ScrollView {
                    id: discoverScroll
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    visible: root.page === "discover"
                    contentWidth: availableWidth
                    ColumnLayout {
                        width: discoverScroll.availableWidth
                        spacing: 10
                        SafeLabel {
                            visible: root.visibleEntries.length === 0
                            text: root.labels.emptyCatalog
                            color: "#a9bed8"
                            wrapMode: Text.WordWrap
                        }
                        Repeater {
                            objectName: "discoverRepeater"
                            model: root.visibleEntries
                            delegate: Rectangle {
                                required property var modelData
                                Layout.fillWidth: true
                                implicitHeight: 106
                                radius: 13
                                color: "#1b2d45"
                                border.color: "#34506f"
                                RowLayout {
                                    anchors.fill: parent
                                    anchors.margins: 14
                                    spacing: 14
                                    Rectangle {
                                        width: 50; height: 50; radius: 12
                                        color: root.backendKey(modelData) === "compatforge" ? "#386fd1" :
                                               root.backendKey(modelData) === "flatpak" ? "#8059bd" : "#c27b36"
                                        SafeLabel { anchors.centerIn: parent; text: root.localized(modelData.name).slice(0, 1).toUpperCase(); color: "white"; font.pixelSize: 24; font.bold: true }
                                    }
                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        SafeLabel { objectName: "appName"; text: root.localized(modelData.name); color: "#f2f6fc"; font.pixelSize: 18; font.bold: true }
                                        SafeLabel { text: root.localized(modelData.summary); color: "#b7c7da"; elide: Text.ElideRight; Layout.fillWidth: true }
                                        SafeLabel {
                                            text: root.backendLabel(root.backendKey(modelData)) + " · " + (modelData.version || "") + " · " + (modelData.license || "")
                                            color: "#87a9d2"; font.pixelSize: 12
                                        }
                                    }
                                    Button {
                                        text: root.labels.details
                                        onClicked: root.openDetails(modelData)
                                        Accessible.name: root.labels.openDetails + " " + root.localized(modelData.name)
                                    }
                                    Button {
                                        text: root.installedRecord(modelData.id) ? root.labels.update : root.labels.install
                                        enabled: root.backendReady(modelData)
                                        onClicked: storeBridge.enqueue(modelData.id, root.installedRecord(modelData.id) ? "update" : "install")
                                        Accessible.name: text + " " + root.localized(modelData.name)
                                    }
                                }
                            }
                        }
                    }
                }

                ScrollView {
                    id: installedScroll
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    visible: root.page === "installed"
                    contentWidth: availableWidth
                    ColumnLayout {
                        width: installedScroll.availableWidth
                        spacing: 10
                        SafeLabel { visible: root.installed.length === 0; text: root.labels.emptyInstalled; color: "#a9bed8" }
                        Repeater {
                            objectName: "installedRepeater"
                            model: root.installed
                            delegate: Rectangle {
                                required property var modelData
                                property var entry: root.findEntry(modelData.appId)
                                Layout.fillWidth: true; implicitHeight: 92; radius: 13
                                color: "#1b2d45"; border.color: "#34506f"
                                RowLayout {
                                    anchors.fill: parent; anchors.margins: 14; spacing: 12
                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        SafeLabel { text: entry ? root.localized(entry.name) : modelData.appId; color: "#f2f6fc"; font.pixelSize: 18; font.bold: true }
                                        SafeLabel { text: root.backendLabel(modelData.backend) + " · " + (modelData.version || root.labels.unknown); color: "#a9bed8" }
                                    }
                                    Button { visible: entry !== null; text: root.labels.details; onClicked: root.openDetails(entry) }
                                    Button { text: root.labels.update; enabled: entry !== null && root.backendReady(entry); onClicked: storeBridge.enqueue(modelData.appId, "update") }
                                    Button { text: root.labels.rollback; enabled: modelData.canRollback === true; onClicked: storeBridge.enqueue(modelData.appId, "rollback") }
                                    Button {
                                        objectName: "installedUninstallButton"
                                        text: root.labels.uninstall
                                        visible: modelData.backend !== "forge-package"
                                        onClicked: storeBridge.enqueue(modelData.appId, "uninstall")
                                    }
                                }
                            }
                        }
                    }
                }

                ScrollView {
                    id: queueScroll
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    visible: root.page === "queue"
                    contentWidth: availableWidth
                    ColumnLayout {
                        width: queueScroll.availableWidth
                        spacing: 10
                        SafeLabel { visible: root.jobs.length === 0; text: root.labels.emptyQueue; color: "#a9bed8" }
                        Repeater {
                            objectName: "queueRepeater"
                            model: root.jobs
                            delegate: Rectangle {
                                required property var modelData
                                Layout.fillWidth: true; implicitHeight: 112; radius: 13
                                color: "#1b2d45"; border.color: "#34506f"
                                RowLayout {
                                    anchors.fill: parent; anchors.margins: 14; spacing: 12
                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        SafeLabel {
                                            text: (root.findEntry(modelData.appId) ? root.localized(root.findEntry(modelData.appId).name) : modelData.appId)
                                                  + " · " + root.statusLabel(modelData.state)
                                            color: "#f2f6fc"; font.pixelSize: 17; font.bold: true
                                        }
                                        SafeLabel { text: root.backendLabel(modelData.backend) + " · " + modelData.action; color: "#9fb8d6" }
                                        SafeLabel { text: modelData.detail || ""; color: "#b7c7da"; elide: Text.ElideRight; Layout.fillWidth: true }
                                        ProgressBar {
                                            Layout.fillWidth: true
                                            visible: ["queued", "preparing", "running", "cancelling"].indexOf(modelData.state) >= 0
                                            indeterminate: true
                                        }
                                    }
                                    Button {
                                        objectName: "queueCancelButton"
                                        text: root.labels.cancel
                                        visible: ["queued", "running"].indexOf(modelData.state) >= 0
                                                 && !(modelData.backend === "forge-package" && modelData.state === "running")
                                        enabled: ["queued", "preparing", "running"].indexOf(modelData.state) >= 0
                                        onClicked: storeBridge.cancel(modelData.id)
                                    }
                                    Button {
                                        objectName: "queueRetryButton"
                                        text: root.labels.retry
                                        enabled: ["interrupted", "failed", "cancelled"].indexOf(modelData.state) >= 0
                                        onClicked: storeBridge.retry(modelData.id)
                                    }
                                }
                            }
                        }
                    }
                }

                ScrollView {
                    id: diagnosticsScroll
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    visible: root.page === "diagnostics"
                    contentWidth: availableWidth
                    ColumnLayout {
                        width: diagnosticsScroll.availableWidth
                        spacing: 12
                        SafeLabel { text: root.labels.partialAvailability; color: "#f2f6fc"; font.pixelSize: 20; font.bold: true }
                        Repeater {
                            objectName: "diagnosticsRepeater"
                            model: Object.keys(root.backends).sort()
                            delegate: Rectangle {
                                required property string modelData
                                property var record: root.backends[modelData]
                                Layout.fillWidth: true; implicitHeight: 74; radius: 12
                                color: "#1b2d45"; border.color: "#34506f"
                                RowLayout {
                                    anchors.fill: parent; anchors.margins: 12
                                    SafeLabel { text: root.backendLabel(modelData); color: "#f2f6fc"; Layout.preferredWidth: 180 }
                                    SafeLabel { text: record.available ? root.labels.available : root.labels.unavailable; color: record.available ? "#8bd9af" : "#ffbd87" }
                                    SafeLabel { text: record.reason || ""; color: "#b7c7da"; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                                }
                            }
                        }
                        SafeLabel { text: root.labels.serviceError; color: "#f2f6fc"; font.pixelSize: 20; font.bold: true }
                        SafeLabel { text: root.displayedError() || root.labels.noServiceError; color: storeBridge.error ? "#ffbd87" : "#a9bed8"; wrapMode: Text.WordWrap; Layout.fillWidth: true }
                    }
                }
            }
        }
    }

    footer: Rectangle {
        height: 34
        color: "#0b101a"
        SafeLabel {
            anchors.fill: parent
            anchors.leftMargin: 24
            verticalAlignment: Text.AlignVCenter
            text: root.displayedError() || root.labels.continueInBackground
            color: storeBridge.error ? "#ffbd87" : "#8da6c2"
            elide: Text.ElideRight
        }
    }

    Drawer {
        id: details
        objectName: "detailsDrawer"
        edge: Qt.RightEdge
        width: Math.min(root.width * 0.42, 430)
        height: root.height
        modal: true
        background: Rectangle { color: "#142135"; border.color: "#34506f" }
        ScrollView {
            id: detailsScroll
            anchors.fill: parent
            anchors.margins: 24
            clip: true
            contentWidth: availableWidth
            ColumnLayout {
                width: detailsScroll.availableWidth
                spacing: 14
                RowLayout {
                    Layout.fillWidth: true
                    SafeLabel { text: root.labels.details; color: "#f2f6fc"; font.pixelSize: 23; font.bold: true; Layout.fillWidth: true }
                    Button { text: root.labels.close; onClicked: details.close() }
                }
                SafeLabel { text: root.selectedEntry ? root.localized(root.selectedEntry.name) : ""; color: "#f2f6fc"; font.pixelSize: 26; font.bold: true; wrapMode: Text.WordWrap; Layout.fillWidth: true }
                SafeLabel { text: root.selectedEntry ? root.localized(root.selectedEntry.summary) : ""; color: "#b7c7da"; wrapMode: Text.WordWrap; Layout.fillWidth: true }
                Repeater {
                    model: root.selectedEntry ? [
                        [root.labels.version, root.selectedEntry.version || root.labels.unknown],
                        [root.labels.publisher, root.selectedEntry.publisher || root.labels.unknown],
                        [root.labels.license, root.selectedEntry.license || root.labels.unknown],
                        [root.labels.origin, root.selectedEntry.origin || root.labels.unknown],
                        [root.labels.backend, root.backendLabel(root.backendKey(root.selectedEntry))],
                        [root.labels.permissions, root.permissionList(root.selectedEntry) || root.labels.noPermissions],
                        [root.labels.compatibility, root.selectedEntry.compatibility ? (root.selectedEntry.compatibility.status || root.labels.notTested) : root.labels.notTested],
                        [root.labels.evidence, root.selectedEntry.compatibility ? (root.selectedEntry.compatibility.evidence || root.labels.notTested) : root.labels.notTested]
                    ] : []
                    delegate: ColumnLayout {
                        required property var modelData
                        Layout.fillWidth: true
                        SafeLabel { text: modelData[0]; color: "#86a8d0"; font.pixelSize: 12 }
                        SafeLabel { text: modelData[1]; color: "#e2eaf4"; wrapMode: Text.WordWrap; Layout.fillWidth: true }
                    }
                }
                SafeLabel {
                    visible: root.selectedEntry !== null && !root.backendReady(root.selectedEntry)
                    text: root.selectedEntry ? root.backendReason(root.selectedEntry) : ""
                    color: "#ffbd87"; wrapMode: Text.WordWrap; Layout.fillWidth: true
                }
                RowLayout {
                    Layout.fillWidth: true
                    Button {
                        text: root.selectedEntry && root.installedRecord(root.selectedEntry.id) ? root.labels.update : root.labels.install
                        enabled: root.selectedEntry !== null && root.backendReady(root.selectedEntry)
                        onClicked: { storeBridge.enqueue(root.selectedEntry.id, root.installedRecord(root.selectedEntry.id) ? "update" : "install"); details.close(); root.page = "queue" }
                    }
                    Button {
                        objectName: "detailsUninstallButton"
                        text: root.labels.uninstall
                        visible: root.selectedEntry !== null && root.installedRecord(root.selectedEntry.id) !== null
                                 && root.backendKey(root.selectedEntry) !== "forgePackage"
                        onClicked: { storeBridge.enqueue(root.selectedEntry.id, "uninstall"); details.close(); root.page = "queue" }
                    }
                }
            }
        }
    }
}
