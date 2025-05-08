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
                spacing: 0

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

                    FormCard.FormRadioSelectorDelegate {
                        consistentWidth: true
                        actions: [
                            Kirigami.Action {
                                id: nameAction
                                text: i18nc("@option:radio", "Name")
                            },
                            Kirigami.Action {
                                id: idAction
                                text: i18nc("@option:radio", "ID")
                            }
                        ]
                    }

                    FormCard.AbstractFormDelegate {
                        id: inputDelegate

                        contentItem: QQC2.TextField {
                            id: inputField
                            placeholderText: nameAction.checked ? i18nc("@info:placeholder", "Character name") : i18nc("@info:placeholder", "Lodestone ID")
                            focus: true
                        }
                    }

                    FormCard.FormDelegateSeparator {
                        above: inputDelegate
                        below: dalamudCheckbox
                    }

                    FormCard.FormCheckDelegate {
                        id: dalamudCheckbox
                        text: i18n("Connect to the Dalamud Plugin")
                    }

                    FormCard.FormDelegateSeparator {
                        above: dalamudCheckbox
                        below: loginButton
                    }

                    FormCard.FormButtonDelegate {
                        id: loginButton
                        icon.name: "cloud-download-symbolic"
                        text: i18nc("@action:button", "Archive")
                        enabled: inputField.text.length > 0
                        onClicked: {
                            fileDialog.selectedFile = inputField.text;
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
                        onClicked: applicationWindow().pageStack.layers.push(Qt.createComponent("org.kde.kirigamiaddons.formcard", "AboutPage"))
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
            if (nameAction.checked) {
                root.backend.archiveCharacterByName(inputField.text, dalamudCheckbox.checked, path);
            } else {
                root.backend.archiveCharacterById(inputField.text, dalamudCheckbox.checked, path);
            }
            root.lastArchiveFile = path;
        }
    }
}
