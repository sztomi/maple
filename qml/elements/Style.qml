pragma Singleton
import QtQuick 2.0
import QtQml 2.2

QtObject {
  property int textSize: 20
  property QtObject colors: QtObject {
    property color white: "#fff"
    property color charcoal: "#264653"
    property color persianGreen: "#2A9D8F"
    property color orangeYellowCrayola: "#E9C46A"
    property color sandyBrown: "#F4A261"
    property color burntSienna: "#E76F51"
    property color blueMunsell: "#1B9AAA"
    property color richBlack: "#050505"
  }
}
