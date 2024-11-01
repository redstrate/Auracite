import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import zone.xiv.auracite

Kirigami.ApplicationWindow {
    id: root

    title: "Auracite"

    readonly property Backend backend: Backend {}

    ColumnLayout {
        QQC2.TextField {
            id: characterNameField

            placeholderText: "Full name of the character"
        }

        QQC2.Button {
            text: "Archive"
            onClicked: root.backend.archiveCharacter(characterNameField.text, false)
        }
    }

    Connections {
        target: backend

        function onArchiveSuccessful(): void {
            console.info("Archive done!");
        }

        function onArchiveFailed(message: string): void {
            console.error("Failed: " + message);
        }
    }
}