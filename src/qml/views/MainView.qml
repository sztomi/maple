import QtQuick 2.0
import "../elements" as Maple


Item {
  id: mainView
  anchors.centerIn: parent

  Component.onCompleted: {
    console.log("hello I'm mainview")
  }
}
