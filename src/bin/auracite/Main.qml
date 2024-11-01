import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.formcard as FormCard
import zone.xiv.auracite

Kirigami.ApplicationWindow {
    id: root

    title: "Auracite"

    readonly property Backend backend: Backend {}

    pageStack.initialPage: Kirigami.Page {
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
                    onClicked: root.backend.archiveCharacter(characterNameField.text, dalamudCheckbox.checked)
                }
            }

            FormCard.FormCard {
                Layout.topMargin: Kirigami.Units.largeSpacing

                maximumWidth: Kirigami.Units.gridUnit * 20

                FormCard.FormButtonDelegate {
                    id: aboutButton
                    text: i18nc("@action:button Application settings", "Settings")
                    icon.name: "settings-configure"
                    onClicked: applicationWindow().pageStack.layers.push(aboutPage)

                    Component {
                        id: aboutPage
                        FormCard.AboutPage {}
                    }
                }
            }
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