import QtQuick 2.6
import QtQuick.Window 2.0
import QtQuick.Layouts 1.1
import QtQuick.Controls 2.0

import Greeter 1.0

Item {
    id: root
    
    property var name;

    Window {
        visible: true
        width: 600
        height: 600
        minimumWidth: 250
        minimumHeight: 200

        Greeter {
            id: greeter
            name: "World"
        }
        
        ColumnLayout {
            id: layout
            spacing: 10
            anchors.fill: parent

            TextField  {
                id: name
                font.pointSize: 15
                Layout.fillWidth: true
                focus: true
                color: "red"
                
                background: Rectangle {
                    radius: 2
                    implicitWidth: 200
                    implicitHeight: 30
                    border.color: "green"
                    border.width: 1
              }
            }

            Button {
                text: "Greet!"
                font.pointSize: 15
                Layout.fillWidth: true

                onClicked: {
                    root.name = name.text
                    greeter.print_hello(root.name)
                }
            }

            Text {
                font.pointSize: 15
                Layout.fillWidth: true
                text: greeter.compute_greetings("hello", root.name)
            }
        }
    }
}