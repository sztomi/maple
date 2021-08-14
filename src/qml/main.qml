import QtQuick 2.0
import QtQuick.Window 2.2
import QtQuick.Controls 2.15
import "views" as Views
import "elements" as Maple
import MapleNative 1.0

ApplicationWindow {
  id: appWindow
  visible: true
  width: 1024
  height: 600
  title: qsTr("Maple for Plex")
  color: Maple.Style.colors.richBlack

  Loader {
    id: currentView
  }

  StackView {
    id: stack
    anchors.centerIn: parent
    initialItem: currentView
  }

  Component.onCompleted: {
    if (dispatcher.get_app_state() == AppState.LoggedIn) {
      currentView.source = "views/MainView.qml"
    }
    else {
      currentView.source = "views/LoginView.qml"
    }
  }
}

/*##^##
Designer {
    D{i:0;formeditorZoom:0.75}
}
##^##*/
