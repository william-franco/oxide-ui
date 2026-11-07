//! Visual showcase of the Oxide components: `cargo run --example gallery`.

slint::slint! {
    import { ScrollView } from "std-widgets.slint";
    import {
        OxideTheme, OxideType, OxideSpacing, OxideIcons,
        OxideButton, OxideButtonVariant, OxideIconButton, OxideIconButtonVariant,
        OxideDisplay, OxidePanel, OxideSurfaceTone, OxideListItem, OxideSegmented, OxideDivider,
        OxideTextField, OxideSearchField, OxideChip, OxideChipVariant, OxideCheckbox, OxideSwitch,
        OxideSelect, OxideDialog, OxideDialogAction, OxideFab, OxideSnackbar,
        OxideDatePicker, OxideTimePicker,
    } from "ui/oxide.slint";

    export component Gallery inherits Window {
        title: "Oxide UI gallery";
        preferred-width: 720px;
        preferred-height: 820px;
        background: OxideTheme.colors.surface;

        property <bool> dialog-shown: false;
        property <bool> snack-shown: false;
        property <string> query;

        ScrollView {
            VerticalLayout {
                padding: OxideSpacing.xl;
                spacing: OxideSpacing.lg;

                HorizontalLayout {
                    spacing: OxideSpacing.sm;
                    Text {
                        text: "Oxide UI";
                        font-size: OxideType.headline-medium.size;
                        color: OxideTheme.colors.on-surface;
                        vertical-alignment: center;
                    }
                    Rectangle { }
                    OxideIconButton {
                        icon: OxideTheme.dark ? OxideIcons.light-mode : OxideIcons.dark-mode;
                        description: "Toggle theme";
                        clicked => { OxideTheme.dark = !OxideTheme.dark; }
                    }
                }

                HorizontalLayout {
                    spacing: OxideSpacing.sm;
                    OxideButton { text: "Filled"; variant: OxideButtonVariant.filled; }
                    OxideButton { text: "Tonal"; variant: OxideButtonVariant.tonal; }
                    OxideButton { text: "Outlined"; variant: OxideButtonVariant.outlined; }
                    OxideButton { text: "Text"; variant: OxideButtonVariant.text; }
                }

                HorizontalLayout {
                    spacing: OxideSpacing.sm;
                    OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.filled; description: "Filled"; }
                    OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.tonal; description: "Tonal"; }
                    OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.outlined; description: "Outlined"; }
                    OxideSegmented { options: ["DEG", "RAD"]; description: "Angle"; }
                    OxideFab { icon: OxideIcons.add; description: "Novo"; }
                    OxideFab { icon: OxideIcons.add; text: "Novo evento"; }
                }

                OxidePanel {
                    tone: OxideSurfaceTone.high;
                    level: 1;
                    height: 120px;
                    OxideDisplay {
                        width: 100%;
                        height: 100%;
                        caption: "CAPTION";
                        primary: "12 × (3 + 4)";
                        secondary: "84";
                    }
                }

                OxideSearchField {
                    text <=> root.query;
                    placeholder: "Pesquisar eventos";
                }

                HorizontalLayout {
                    spacing: OxideSpacing.sm;
                    OxideChip { text: "Trabalho"; accent: 0; selected: true; }
                    OxideChip { text: "Pessoal"; accent: 2; }
                    OxideChip { text: "Saúde"; accent: 4; variant: OxideChipVariant.input; }
                    OxideChip { text: "Lembrete"; variant: OxideChipVariant.assist; }
                }

                HorizontalLayout {
                    spacing: OxideSpacing.lg;
                    OxideCheckbox { text: "Mostrar concluídos"; checked: true; }
                    OxideSwitch { text: "Dia inteiro"; }
                }

                OxideTextField {
                    label: "Título";
                    placeholder: "Ex.: Reunião de planejamento";
                    supporting: "Obrigatório";
                    error: true;
                }

                OxideSelect {
                    label: "Categoria";
                    options: ["Trabalho", "Pessoal", "Saúde", "Estudos"];
                    selected: 0;
                }

                HorizontalLayout {
                    spacing: OxideSpacing.xl;
                    OxideDatePicker {
                        year: 2026;
                        month: 11;
                        selected-year: 2026;
                        selected-month: 11;
                        selected-day: 7;
                        today-year: 2026;
                        today-month: 11;
                        today-day: 7;
                        width: 320px;
                    }
                    OxideTimePicker {
                        hour: 9;
                        minute: 30;
                        width: 240px;
                    }
                }

                OxideButton {
                    text: "Mostrar diálogo";
                    variant: OxideButtonVariant.tonal;
                    clicked => { root.dialog-shown = true; }
                }

                OxideButton {
                    text: "Mostrar snackbar";
                    variant: OxideButtonVariant.outlined;
                    clicked => { root.snack-shown = true; }
                }
            }
        }

        OxideDialog {
            width: 100%;
            height: 100%;
            shown: root.dialog-shown;
            headline: "Excluir evento?";
            dismissed => { root.dialog-shown = false; }

            Text {
                text: "Esta ação não pode ser desfeita.";
                font-size: OxideType.body-large.size;
                color: OxideTheme.colors.on-surface-variant;
                wrap: word-wrap;
            }

            HorizontalLayout {
                alignment: end;
                spacing: OxideSpacing.sm;
                OxideDialogAction { text: "Cancelar"; clicked => { root.dialog-shown = false; } }
                OxideDialogAction { text: "Excluir"; primary: true; clicked => { root.dialog-shown = false; } }
            }
        }

        OxideSnackbar {
            width: 100%;
            height: 100%;
            shown: root.snack-shown;
            message: "Evento excluído";
            action-text: "Desfazer";
            action => { root.snack-shown = false; }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    Gallery::new()?.run()
}
