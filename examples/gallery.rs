//! Visual showcase of the Oxide components: `cargo run --example gallery`.

slint::slint! {
    import {
        OxideTheme, OxideType, OxideSpacing, OxideIcons,
        OxideButton, OxideButtonVariant, OxideIconButton, OxideIconButtonVariant,
        OxideDisplay, OxidePanel, OxideSurfaceTone, OxideListItem, OxideSegmented, OxideDivider,
    } from "ui/oxide.slint";

    export component Gallery inherits Window {
        title: "Oxide UI gallery";
        preferred-width: 720px;
        preferred-height: 640px;
        background: OxideTheme.colors.surface;

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
                OxideButton { text: "Tertiary"; variant: OxideButtonVariant.tertiary; }
                OxideButton { text: "Elevated"; variant: OxideButtonVariant.elevated; }
                OxideButton { text: "Outlined"; variant: OxideButtonVariant.outlined; }
                OxideButton { text: "Text"; variant: OxideButtonVariant.text; }
            }

            HorizontalLayout {
                spacing: OxideSpacing.sm;
                OxideButton { text: "Icon"; icon: OxideIcons.check; }
                OxideButton { text: "Disabled"; enabled: false; }
                OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.filled; description: "Filled"; }
                OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.tonal; description: "Tonal"; }
                OxideIconButton { icon: OxideIcons.history; variant: OxideIconButtonVariant.outlined; description: "Outlined"; }
                OxideSegmented { options: ["DEG", "RAD"]; description: "Angle"; }
            }

            OxidePanel {
                tone: OxideSurfaceTone.high;
                level: 1;
                height: 140px;
                OxideDisplay {
                    width: 100%;
                    height: 100%;
                    caption: "CAPTION";
                    primary: "12 × (3 + 4)";
                    secondary: "84";
                }
            }

            OxidePanel {
                tone: OxideSurfaceTone.low;
                VerticalLayout {
                    padding: OxideSpacing.sm;
                    OxideListItem { headline: "84"; supporting: "12 × (3 + 4)"; text-alignment: right; }
                    OxideDivider { }
                    OxideListItem { headline: "One-line item"; }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    Gallery::new()?.run()
}
