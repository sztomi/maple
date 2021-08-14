import QtQuick 2.0
import QtGraphicalEffects 1.12
import "../elements" as Maple

Item {
  id: mainView
  anchors.fill: parent
  width: 1024
  height: 600

  Rectangle {
    id: leftMenu
    width: 260
    height: parent.height
    anchors.left: parent.left
    anchors.top: topbar.bottom
    color: Maple.Style.colors.white
  }

  Rectangle {
    id: listView
    anchors.top: topbar.bottom
    anchors.left: leftMenu.right
    anchors.right: parent.right
    anchors.bottom: parent.bottom
    color: Maple.Style.colors.sandyBrown
  }

  Rectangle {
    id: topbar
    width: parent.width
    height: 75
    color: Maple.Style.colors.charcoal
  }

  DropShadow {
    anchors.fill: topbar
    verticalOffset: 3
    radius: 6.0
    samples: 17
    color: "#80000000"
    source: topbar
  }




  Component.onCompleted: {
    console.log("hello I'm mainview")
  }
}


