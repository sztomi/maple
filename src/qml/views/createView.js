
var component;

function createView(viewFileName, stack) {
  component = Qt.createComponent(viewFileName);
  if (component.status == Component.Ready) {
    stack.push(component.createObject(appWindow, {x: 100, y: 100}))
  }
  else {
    component.statusChanged.connect(function() {
      if (component.status == Component.Ready) {
        stack.push(component.createObject(appWindow, {x: 100, y: 100}))
      } else if (component.status == Component.Error) {
        console.log("Error loading component:", component.errorString());
      }
    })
  }
}
