import QtQuick
import QtQuick.Controls
import QtQuick.Window

Item {
    id: chrome
    required property var targetWindow
    required property var labels
    property bool titleGesturesEnabled: true
    objectName: "windowChrome"
    property var effects
    property color foreground: "#f2f6fc"
    implicitHeight: 40

    function titleDoubleClick() {
        if (!enabled || !titleGesturesEnabled || targetWindow.visibility === Window.FullScreen) return
        if (Qt.platform.os === "osx") {
            if (effects) effects.titlebarDoubleClick()
        } else maximizeOrRestore()
    }
    function startTitleDrag() {
        if (!enabled || !titleGesturesEnabled || targetWindow.visibility === Window.FullScreen) return
        targetWindow.startSystemMove()
    }

    function maximizeOrRestore() {
        if (targetWindow.visibility === Window.Maximized || targetWindow.visibility === Window.FullScreen)
            targetWindow.showNormal()
        else targetWindow.showMaximized()
    }

    // Only the title and empty strip drag; controls and their focus rings are outside it.
    Item {
        objectName: "windowDragRegion"
        anchors.left: parent.left
        anchors.right: controls.left
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.margins: 8
        Label {
            anchors.fill: parent
            anchors.leftMargin: 8
            text: chrome.labels.title
            color: chrome.foreground
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
            textFormat: Text.PlainText
        }
        TapHandler {
            acceptedButtons: Qt.LeftButton
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onDoubleTapped: chrome.titleDoubleClick()
        }
        DragHandler {
            target: null
            acceptedButtons: Qt.LeftButton
            onActiveChanged: if (active) chrome.startTitleDrag()
        }
    }
    Row {
        id: controls
        anchors.right: parent.right
        anchors.rightMargin: 10
        anchors.top: parent.top
        // Keep every control below the 8px resize band, with a 2px gap.
        anchors.topMargin: 10
        spacing: 4
        Button {
            objectName: "windowEffectsButton"
            focusPolicy: Qt.StrongFocus
            width: 34; height: 28
            text: "◈"
            enabled: !!chrome.effects
            checkable: true
            checked: chrome.effects ? chrome.effects.enabled : false
            Accessible.name: checked ? chrome.labels.effectsOff : chrome.labels.effectsOn
            ToolTip.visible: hovered
            ToolTip.text: Accessible.name
            onClicked: chrome.effects.setEnabled(checked)
        }
        Button {
            objectName: "windowMinimizeButton"
            focusPolicy: Qt.StrongFocus
            width: 34; height: 28
            text: "−"
            Accessible.name: chrome.labels.minimize
            onClicked: chrome.targetWindow.showMinimized()
        }
        Button {
            objectName: "windowMaximizeButton"
            focusPolicy: Qt.StrongFocus
            width: 34; height: 28
            text: chrome.targetWindow.visibility === Window.Windowed ? "□" : "↙"
            Accessible.name: chrome.targetWindow.visibility === Window.FullScreen ? chrome.labels.exitFullscreen
                : chrome.targetWindow.visibility === Window.Maximized ? chrome.labels.restore : chrome.labels.maximize
            onClicked: chrome.maximizeOrRestore()
        }
        Button {
            objectName: "windowCloseButton"
            focusPolicy: Qt.StrongFocus
            width: 34; height: 28
            text: "×"
            Accessible.name: chrome.labels.close
            // Preserve normal last-window-close exit; the service owns background jobs.
            onClicked: chrome.targetWindow.close()
        }
    }
}
