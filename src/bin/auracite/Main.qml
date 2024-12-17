import QtCore
import QtQuick
import QtQuick.Layouts
import QtQuick.Dialogs
import QtQuick.Controls as QQC2

import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.formcard as FormCard

import zone.xiv.auracite

Kirigami.ApplicationWindow {
    id: root

    readonly property Backend backend: Backend {}

    property string lastArchiveFile

    property Kirigami.Action openArchiveAction: Kirigami.Action {
        text: i18nc("@action:button", "Open Archive")
        icon.name: "document-open"
        onTriggered: Qt.openUrlExternally("file://" + root.lastArchiveFile)
    }

    pageStack {
        defaultColumnWidth: root.width

        initialPage: Kirigami.Page {
            globalToolBarStyle: Kirigami.ApplicationHeaderStyle.None

            header: ColumnLayout {
                Kirigami.Separator {
                    Layout.fillWidth: true
                }

                Kirigami.InlineMessage {
                    id: messageBanner

                    position: Kirigami.InlineMessage.Position.Header
                    actions: type === Kirigami.MessageType.Information ? [openArchiveAction] : []

                    Layout.fillWidth: true
                }
            }

            contentItem: ColumnLayout {
                anchors {
                    left: parent.left
                    right: parent.right
                    verticalCenter: parent.verticalCenter
                }

                spacing: Kirigami.Units.largeSpacing

                FormCard.FormCard {
                    Layout.topMargin: Kirigami.Units.largeSpacing

                    maximumWidth: Kirigami.Units.gridUnit * 20

                    FormCard.FormTextFieldDelegate {
                        id: characterNameField
                        label: i18n("Character Name")
                        placeholderText: "Full name of the character"
                        focus: true
                    }

                    FormCard.FormDelegateSeparator {}

                    FormCard.FormCheckDelegate {
                        id: dalamudCheckbox
                        text: i18n("Use Dalamud Plugin")
                    }

                    FormCard.FormDelegateSeparator {}

                    FormCard.FormButtonDelegate {
                        id: loginButton
                        text: i18nc("@action:button", "Archive")
                        enabled: characterNameField.text.length > 0
                        onClicked: {
                            fileDialog.selectedFile = characterNameField.text;
                            fileDialog.open();
                        }
                    }
                }

                FormCard.FormCard {
                    Layout.topMargin: Kirigami.Units.largeSpacing

                    maximumWidth: Kirigami.Units.gridUnit * 20

                    FormCard.FormButtonDelegate {
                        id: aboutButton
                        text: i18nc("@action:button Application settings", "Settings")
                        icon.name: "settings-configure"
                        onClicked: applicationWindow().pageStack.push(Qt.createComponent("org.kde.kirigamiaddons.formcard", "AboutPage"))
                    }
                }
            }
        }
    }

    Connections {
        target: backend

        function onArchiveSuccessful(): void {
            messageBanner.type = Kirigami.MessageType.Information;
            messageBanner.text = i18n("Archive completed!");
            messageBanner.visible = true;
        }

        function onArchiveFailed(message: string): void {
            messageBanner.type = Kirigami.MessageType.Error;
            messageBanner.text = message;
            messageBanner.visible = true;
        }
    }

    FileDialog {
        id: fileDialog
        fileMode: FileDialog.SaveFile
        nameFilters: ["ZIP files (*.zip)"]
        currentFolder: StandardPaths.standardLocations(StandardPaths.DocumentsLocation)[0]
        onAccepted: {
            let path = selectedFile.toString();
            // Remove file://
            path = path.replace(/^(file:\/{2})/,"");
            root.backend.archiveCharacter(characterNameField.text, dalamudCheckbox.checked, path);
            root.lastArchiveFile = path;
        }
    }
}