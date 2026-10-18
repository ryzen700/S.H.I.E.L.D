use crate::theme::HydraTheme;
use gpui::{prelude::*, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Success,
    Warning,
    Info,
    Brand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationTab {
    Catalogue,
    Library,
    Downloads,
    Settings,
}

pub fn render_button(label: &'static str, variant: ButtonVariant) -> Div {
    let theme = HydraTheme::default();

    let (bg, bg_hover, text_color, border_color) = match variant {
        ButtonVariant::Primary => (
            theme.brand_teal,
            theme.platinum_cyan,
            rgba(0x000000ff).into(),
            theme.brand_teal,
        ),
        ButtonVariant::Secondary => (
            theme.bg_card,
            theme.bg_hover,
            theme.text_muted,
            theme.border,
        ),
        ButtonVariant::Outline => (
            hsla(0.0, 0.0, 0.0, 0.0),
            theme.bg_hover,
            theme.text_body,
            theme.border,
        ),
        ButtonVariant::Danger => (
            theme.error,
            rgba(0xbe123cff).into(),
            rgba(0xffffffff).into(),
            theme.error,
        ),
    };

    div()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .rounded_md()
        .border_1()
        .border_color(border_color)
        .bg(bg)
        .text_color(text_color)
        .font_weight(FontWeight::SEMIBOLD)
        .text_size(AbsoluteLength::Pixels(px(13.0)))
        .cursor_pointer()
        .hover(move |this| this.bg(bg_hover))
        .child(label)
}

pub fn render_badge(text: impl Into<SharedString>, variant: BadgeVariant) -> Div {
    let theme = HydraTheme::default();

    let (bg, text_color, border_color) = match variant {
        BadgeVariant::Default => (
            theme.bg_card,
            theme.text_body,
            theme.border,
        ),
        BadgeVariant::Success => (
            rgba(0x1c974926).into(),
            theme.success,
            theme.success,
        ),
        BadgeVariant::Warning => (
            rgba(0xffc10726).into(),
            theme.warning,
            theme.warning,
        ),
        BadgeVariant::Info => (
            rgba(0x3e62c026).into(),
            theme.brand_blue,
            theme.brand_blue,
        ),
        BadgeVariant::Brand => (
            rgba(0x16b19526).into(),
            theme.brand_teal,
            theme.brand_teal,
        ),
    };

    div()
        .flex()
        .items_center()
        .px_2()
        .py_0p5()
        .rounded_sm()
        .border_1()
        .border_color(border_color)
        .bg(bg)
        .text_color(text_color)
        .text_size(AbsoluteLength::Pixels(px(11.0)))
        .font_weight(FontWeight::MEDIUM)
        .child(text.into())
}

pub fn render_card(
    title: &str,
    platform: &str,
    is_installed: bool,
    playtime_hours: Option<f32>,
) -> Div {
    let theme = HydraTheme::default();

    div()
        .flex()
        .flex_col()
        .w(px(180.0))
        .rounded_lg()
        .overflow_hidden()
        .border_1()
        .border_color(theme.border)
        .bg(theme.bg_card)
        .cursor_pointer()
        .hover(move |this| this.border_color(theme.brand_teal).bg(theme.bg_hover))
        // Poster area (180x240)
        .child(
            div()
                .w_full()
                .h(px(240.0))
                .bg(theme.bg_dark)
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(theme.text_secondary)
                        .text_size(AbsoluteLength::Pixels(px(14.0)))
                        .font_weight(FontWeight::BOLD)
                        .child(title.to_string())
                )
                .when(is_installed, |this| {
                    this.child(
                        div()
                            .absolute()
                            .top_2()
                            .right_2()
                            .child(render_badge("INSTALLED", BadgeVariant::Success))
                    )
                })
        )
        // Metadata footer
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .gap_1()
                .child(
                    div()
                        .text_color(theme.text_muted)
                        .font_weight(FontWeight::BOLD)
                        .text_size(AbsoluteLength::Pixels(px(13.0)))
                        .overflow_hidden()
                        .child(title.to_string())
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(render_badge(platform.to_string(), BadgeVariant::Default))
                        .when_some(playtime_hours, |this, hours| {
                            this.child(
                                div()
                                    .text_color(theme.text_secondary)
                                    .text_size(AbsoluteLength::Pixels(px(11.0)))
                                    .child(format!("{:.1}h", hours))
                            )
                        })
                )
        )
}

pub fn render_header(search_query: &str) -> Div {
    let theme = HydraTheme::default();

    div()
        .flex()
        .items_center()
        .justify_between()
        .w_full()
        .h(px(56.0))
        .px_6()
        .bg(theme.bg_dark)
        .border_b_1()
        .border_color(theme.border)
        // Search bar
        .child(
            div()
                .flex()
                .items_center()
                .w(px(320.0))
                .px_3()
                .py_1p5()
                .rounded_md()
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border)
                .child(
                    div()
                        .text_color(theme.text_secondary)
                        .text_size(AbsoluteLength::Pixels(px(13.0)))
                        .child(if search_query.is_empty() {
                            "Search in catalogue & library...".to_string()
                        } else {
                            search_query.to_string()
                        })
                )
        )
        // Status & indicator
        .child(
            div()
                .flex()
                .items_center()
                .gap_4()
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.bg_card)
                        .text_color(theme.text_secondary)
                        .text_size(AbsoluteLength::Pixels(px(11.5)))
                        .child("v0.1.0-shield")
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w(px(12.0))
                                .h(px(12.0))
                                .rounded_full()
                                .bg(theme.brand_teal)
                        )
                        .child(
                            div()
                                .text_color(theme.text_body)
                                .text_size(AbsoluteLength::Pixels(px(12.0)))
                                .child("GPU Native 120 FPS")
                        )
                )
        )
}

pub fn render_sidebar(
    active_tab: NavigationTab,
    installed_count: usize,
    downloads_count: usize,
) -> Div {
    let theme = HydraTheme::default();

    let render_item = move |tab: NavigationTab, label: &'static str, count: Option<usize>| {
        let is_active = active_tab == tab;
        let bg = if is_active { rgba(0x16b19526).into() } else { hsla(0.0, 0.0, 0.0, 0.0) };
        let text_color = if is_active { theme.brand_teal } else { theme.text_body };
        let font_weight = if is_active { FontWeight::BOLD } else { FontWeight::MEDIUM };
        let border_l = if is_active { theme.brand_teal } else { hsla(0.0, 0.0, 0.0, 0.0) };

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .px_4()
            .py_2p5()
            .rounded_r_md()
            .border_l_4()
            .border_color(border_l)
            .bg(bg)
            .cursor_pointer()
            .hover(move |this| this.bg(theme.bg_hover))
            .child(
                div()
                    .text_color(text_color)
                    .font_weight(font_weight)
                    .text_size(AbsoluteLength::Pixels(px(13.5)))
                    .child(label)
            )
            .when_some(count, move |this, c| {
                if c > 0 {
                    this.child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_full()
                            .bg(if is_active { theme.brand_teal } else { theme.bg_card })
                            .text_color(if is_active { rgba(0x000000ff).into() } else { theme.text_secondary })
                            .text_size(AbsoluteLength::Pixels(px(10.0)))
                            .font_weight(FontWeight::BOLD)
                            .child(c.to_string())
                    )
                } else {
                    this
                }
            })
    };

    div()
        .flex()
        .flex_col()
        .w(px(240.0))
        .h_full()
        .bg(theme.bg_dark)
        .border_r_1()
        .border_color(theme.border)
        // Logo
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_5()
                .py_6()
                .border_b_1()
                .border_color(theme.border)
                .child(
                    div()
                        .w(px(28.0))
                        .h(px(28.0))
                        .rounded_md()
                        .bg(theme.brand_teal)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgba(0x000000ff))
                        .font_weight(FontWeight::BLACK)
                        .text_size(AbsoluteLength::Pixels(px(14.0)))
                        .child("S")
                )
                .child(
                    div()
                        .text_color(theme.text_muted)
                        .font_weight(FontWeight::BOLD)
                        .text_size(AbsoluteLength::Pixels(px(15.0)))
                        .child("S.H.I.E.L.D")
                )
        )
        // Navigation links
        .child(
            div()
                .flex()
                .flex_col()
                .py_4()
                .gap_1()
                .child(render_item(NavigationTab::Catalogue, "Catalogue", None))
                .child(render_item(NavigationTab::Library, "Library", Some(installed_count)))
                .child(render_item(NavigationTab::Downloads, "Downloads", Some(downloads_count)))
                .child(render_item(NavigationTab::Settings, "Settings", None))
        )
        // User profile pill
        .child(
            div()
                .mt_auto()
                .p_4()
                .border_t_1()
                .border_color(theme.border)
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(34.0))
                        .h(px(34.0))
                        .rounded_full()
                        .bg(theme.brand_blue)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgba(0xffffffff))
                        .font_weight(FontWeight::BOLD)
                        .child("U")
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_size(AbsoluteLength::Pixels(px(12.5)))
                                .child("User Profile")
                        )
                        .child(
                            div()
                                .text_color(theme.brand_teal)
                                .text_size(AbsoluteLength::Pixels(px(10.5)))
                                .child("● Online")
                        )
                )
        )
}
