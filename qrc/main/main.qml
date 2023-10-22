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

            QMLineInput {
                id: inp

                onClicked: {
                    root.name = inp.inputField.text
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