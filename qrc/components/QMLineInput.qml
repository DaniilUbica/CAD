import QtQuick 2.6
import QtQuick.Layouts 1.1
import QtQuick.Controls 2.0

QMLineInput {
    id: root

    RowLayout{
        TextInput {
            id: inputField
            font.pointSize: 15
            Layout.fillWidth: true
            focus: true
            color: "red"
                
            background: Rectangle {
                radius: 2
                implicitWidth: 100
                implicitHeight: 30
                border.color: "black"
                border.width: 1
            }  
        }

        Buttton {
            id: confirmButton  

            text: "confirm"
            font.pointSize: 15
            Layout.fillWidth: true

            onClicked: {}
        }
    }
}
