import QtQuick 2.0
import "../elements" as Maple


Item {
  id: loginView
  anchors.centerIn: parent
  Maple.Button {
    id: loginButton
    text: "Login"
    width: 200
    anchors.centerIn: parent
    onClicked: dispatcher.begin_login()
  }

  Component.onCompleted: {
    console.log("hello I'm loginview")
  }
}