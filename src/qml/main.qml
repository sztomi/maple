import QtQuick 2.0
import QtQuick.Window 2.2
import QtQuick.Controls 2.15
import "views" as Views
import "elements" as Maple
import MapleNative 1.0
import "views/createView.js" as Functions

ApplicationWindow {
  id: appWindow
  visible: true
  width: 1024
  height: 600
  title: qsTr("Maple for Plex")
  color: Maple.Style.colors.richBlack

  StackView {
    id: stack
    anchors.centerIn: parent
  }

  Component.onCompleted: {
    if (dispatcher.get_login_state() == LoginState.LoggedIn) {
      Functions.createView("MainView.qml", stack)
    }
    else {
      Functions.createView("LoginView.qml", stack)
    }
  }
}

/*##^##
Designer {
    D{i:0;formeditorZoom:0.75}
}
##^##*/
