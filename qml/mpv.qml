import QtQuick 2.0
import QtQuick.Window 2.0
import QtQuick.Controls 1.0

import mpvtest 1.0

Window {
    visible: true
    width: 640
    height: 480
    title: qsTr("Hello World") //The method qsTr() is used for translations from one language to other.

    Item {
        width: 1280
        height: 720
        anchors.fill: parent

        MpvObject {
            id: renderer
            anchors.fill: parent

            MouseArea {
                anchors.fill: parent
                // onClicked: renderer.command(["loadfile", "test.mkv"])
                onClicked: dispatcher.beginLogin()
            }
        }

        Rectangle {
            id: labelFrame
            anchors.margins: -50
            radius: 5
            color: "white"
            border.color: "black"
            opacity: 0.8
            anchors.fill: box
        }

        Row {
            id: box
            anchors.bottom: renderer.bottom
            anchors.left: renderer.left
            anchors.right: renderer.right
            anchors.margins: 100

            Text {
                anchors.margins: 10
                wrapMode: Text.WordWrap
                text: "QtQuick and mpv are both rendering stuff.\n
                    Click to load test.mkv"
            }

            // Don't take these controls too seriously. They're for testing.
            Column {
                CheckBox {
                    id: checkbox
                    anchors.margins: 10
                    // Heavily filtered means good, right?
                    text: "Make video look like on a Smart TV"
                    onClicked: {
                        if (checkbox.checked) {
                            renderer.setProperty("sharpen", 5.0)
                        } else {
                            renderer.setProperty("sharpen", 0)
                        }
                    }
                }
                Slider {
                    id: slider
                    anchors.margins: 10
                    anchors.left: checkbox.left
                    anchors.right: checkbox.right
                    minimumValue: -100
                    maximumValue:  100
                    value: 0
                    onValueChanged: renderer.setProperty("gamma", slider.value | 0)
                }
            }
        }
    }
}