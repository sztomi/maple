import QtQuick 2.0
import QtQuick.Layouts 1.11
import QtGraphicalEffects 1.12

import "."

Rectangle {
    width: 240
    height: 80
    color: Style.colors.richBlack
    border.color: Style.colors.charcoal
    border.width: 2
    radius: 8
    clip: true
    property int internalSpacing: 16

    Image {
        id: image
        width: 50
        height: 50
        margins: 8

        anchors.left: parent.left
        anchors.leftMargin: internalSpacing
        anchors.verticalCenter: parent.verticalCenter

        source: "https://plex.tv/users/f8ce59dc47173306/avatar?c=1623837296"
        fillMode: Image.PreserveAspectFit
    }

    Text {
        id: name
        color: Style.colors.white
        text: qsTr("text")
        font: Style.headerFont
        anchors.left: image.right
        anchors.leftMargin: internalSpacing
        anchors.verticalCenter: parent.verticalCenter
    }


}





/*##^##
Designer {
    D{i:0;formeditorZoom:2}
}
##^##*/
