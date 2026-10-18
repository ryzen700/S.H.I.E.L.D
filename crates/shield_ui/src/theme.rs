use gpui::{hsla, rgba, Hsla};

/// Design tokens exactly matching Hydra Launcher's SCSS globals (`globals.scss`).
#[derive(Debug, Clone, Copy)]
pub struct HydraTheme {
    /// Main app content background: #121212
    pub bg_main: Hsla,
    /// Sidebar, header and panel background: #0d0d0d
    pub bg_dark: Hsla,
    /// Card and elevated element background: #1c1c1c
    pub bg_card: Hsla,
    /// Hover background: #262626
    pub bg_hover: Hsla,
    /// Border color: rgba(255, 255, 255, 0.08)
    pub border: Hsla,
    /// Focus border: rgba(255, 255, 255, 0.2)
    pub border_focus: Hsla,
    /// Main body font color: #d0d1d7
    pub text_body: Hsla,
    /// Highlighted / header text color: #f0f1f7
    pub text_muted: Hsla,
    /// Subtext / secondary text: rgba(208, 209, 215, 0.6)
    pub text_secondary: Hsla,
    /// Brand teal accent: #16b195
    pub brand_teal: Hsla,
    /// Brand blue accent: #3e62c0
    pub brand_blue: Hsla,
    /// Platinum cyan accent: #30edc5
    pub platinum_cyan: Hsla,
    /// Success green: #1c9749
    pub success: Hsla,
    /// Error red: #e11d48
    pub error: Hsla,
    /// Warning yellow: #ffc107
    pub warning: Hsla,
}

impl Default for HydraTheme {
    fn default() -> Self {
        Self {
            // #121212 -> rgb(18, 18, 18)
            bg_main: rgba(0x121212ff).into(),
            // #0d0d0d -> rgb(13, 13, 13)
            bg_dark: rgba(0x0d0d0dff).into(),
            // #1c1c1c -> rgb(28, 28, 28)
            bg_card: rgba(0x1c1c1cff).into(),
            // #262626 -> rgb(38, 38, 38)
            bg_hover: rgba(0x262626ff).into(),
            // rgba(255, 255, 255, 0.08)
            border: hsla(0.0, 0.0, 1.0, 0.08),
            // rgba(255, 255, 255, 0.20)
            border_focus: hsla(0.0, 0.0, 1.0, 0.20),
            // #d0d1d7 -> rgb(208, 209, 215)
            text_body: rgba(0xd0d1d7ff).into(),
            // #f0f1f7 -> rgb(240, 241, 247)
            text_muted: rgba(0xf0f1f7ff).into(),
            // rgba(208, 209, 215, 0.60)
            text_secondary: hsla(231.0 / 360.0, 0.10, 0.83, 0.60),
            // #16b195 -> rgb(22, 177, 149)
            brand_teal: rgba(0x16b195ff).into(),
            // #3e62c0 -> rgb(62, 98, 192)
            brand_blue: rgba(0x3e62c0ff).into(),
            // #30edc5 -> rgb(48, 237, 197)
            platinum_cyan: rgba(0x30edc5ff).into(),
            // #1c9749 -> rgb(28, 151, 73)
            success: rgba(0x1c9749ff).into(),
            // #e11d48 -> rgb(225, 29, 72)
            error: rgba(0xe11d48ff).into(),
            // #ffc107 -> rgb(255, 193, 7)
            warning: rgba(0xffc107ff).into(),
        }
    }
}
