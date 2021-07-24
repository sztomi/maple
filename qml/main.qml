import QtQuick 2.0
import QtQuick.Window 2.2
import "elements" as Maple


Window {
    visible: true
    width: 1024
    height: 600
    title: qsTr("Maple for Plex")
    color: Maple.Style.colors.richBlack
    Maple.Button {
      text: "Login"
      width: 200
      anchors.centerIn: parent
      onClicked: dispatcher.begin_login()
    }

}
