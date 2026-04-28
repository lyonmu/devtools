#![allow(dead_code)]

use gpui::{div, rgb, AnyElement, ElementId, InteractiveElement, ParentElement, Rgba, SharedString, StatefulInteractiveElement, Styled};

const fn rgb_const(hex: u32) -> Rgba {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

/// Font size constants for consistent UI typography
pub const FONT_TITLE: gpui::Pixels = gpui::px(18.0);
pub const FONT_BODY: gpui::Pixels = gpui::px(16.0);
pub const FONT_SMALL: gpui::Pixels = gpui::px(14.0);

/// Color constants — single source of truth for the dark theme palette
pub const COLOR_BG_DARK: Rgba = rgb_const(0x1a1a2a);
pub const COLOR_BG_PANEL: Rgba = rgb_const(0x1e1e2e);
pub const COLOR_BG_MENU: Rgba = rgb_const(0x252535);
pub const COLOR_BG_ACTIVE: Rgba = rgb_const(0x3b3b5c);
pub const COLOR_TEXT_PRIMARY: Rgba = rgb_const(0xffffff);
pub const COLOR_TEXT_SECONDARY: Rgba = rgb_const(0x888899);
pub const COLOR_TEXT_MUTED: Rgba = rgb_const(0x666677);
pub const COLOR_TEXT_BODY: Rgba = rgb_const(0xddddcc);
pub const COLOR_BORDER: Rgba = rgb_const(0x3a3a4a);
pub const COLOR_SUCCESS: Rgba = rgb_const(0x22c55e);
pub const COLOR_ERROR: Rgba = rgb_const(0xf87171);
pub const COLOR_INFO: Rgba = rgb_const(0x3b82f6);

/// Color constant for warning severity (amber/yellow)
pub const COLOR_WARNING: Rgba = rgb_const(0xfbbf24);

/// Status banner variants for consistent status display
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiStatusKind {
    Empty,
    Success,
    Error,
    Warning,
    Info,
}

/// Render a status banner with colored left border and icon prefix
pub fn render_status_banner(kind: UiStatusKind, message: impl Into<String>) -> gpui::Div {
    let (border, text, icon) = match kind {
        UiStatusKind::Empty => (rgb(0x888899), rgb(0xaaaabb), ""),
        UiStatusKind::Success => (COLOR_SUCCESS, rgb(0xbbf7d0), ""),
        UiStatusKind::Error => (COLOR_ERROR, rgb(0xfecaca), "⚠ "),
        UiStatusKind::Warning => (COLOR_WARNING, rgb(0xfde68a), ""),
        UiStatusKind::Info => (COLOR_INFO, rgb(0xbfdbfe), ""),
    };
    let display_text = format!("{}{}", icon, message.into());
    div()
        .w_full()
        .px_3()
        .py_2()
        .bg(COLOR_BG_PANEL)
        .border_l_4()
        .border_color(border)
        .rounded_md()
        .text_size(FONT_BODY)
        .text_color(text)
        .child(display_text)
}

/// Render a monospaced output block with scroll
pub fn render_mono_output_block(text: &str) -> gpui::Stateful<gpui::Div> {
    div()
        .w_full()
        .px_3()
        .py_2()
        .bg(COLOR_BG_DARK)
        .rounded_md()
        .id(ElementId::Name(SharedString::from(format!("mono-output-{}", text.len()))))
        .overflow_x_scroll()
        .font_family("monospace")
        .text_size(FONT_SMALL)
        .text_color(COLOR_TEXT_BODY)
        .child(text.to_string())
}

/// Render a result card with title and body
pub fn render_result_card(title: &str, body: AnyElement) -> gpui::Div {
    div()
        .w_full()
        .p_3()
        .bg(COLOR_BG_DARK)
        .border_1()
        .border_color(COLOR_BORDER)
        .rounded_md()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_PRIMARY).child(title.to_string()))
        .child(body)
}

/// Render an action button (caller chains .on_mouse_down())
pub fn render_action_button(id: &str, label: &str, bg_color: Rgba) -> gpui::Stateful<gpui::Div> {
    div()
        .id(ElementId::Name(SharedString::from(id.to_string())))
        .px_4()
        .py_2()
        .bg(bg_color)
        .text_color(COLOR_TEXT_PRIMARY)
        .text_size(FONT_BODY)
        .rounded_md()
        .cursor_pointer()
        .child(label.to_string())
}

/// Render a label-value info row
pub fn render_info_row(label: &str, value: &str) -> gpui::Div {
    div()
        .flex()
        .flex_row()
        .gap_4()
        .py_1()
        .border_b_1()
        .border_color(COLOR_BORDER)
        .child(
            div()
                .w(gpui::px(120.0))
                .text_size(FONT_BODY)
                .text_color(COLOR_TEXT_SECONDARY)
                .child(label.to_string()),
        )
        .child(
            div()
                .flex_1()
                .text_size(FONT_BODY)
                .text_color(COLOR_TEXT_BODY)
                .child(value.to_string()),
        )
}

/// Render a monospaced output block with a copy button
pub fn render_mono_output_block_with_copy(
    text: &str,
    on_copy: impl Fn() + 'static,
) -> gpui::Div {
    div().flex().flex_row().gap_2().items_center()
        .child(render_mono_output_block(text).flex_1())
        .child(
            div()
                .id(ElementId::Name(SharedString::from(format!("copy-{}", text.len()))))
                .px_2().py_1().bg(COLOR_BG_ACTIVE)
                .text_color(COLOR_TEXT_PRIMARY).text_size(FONT_SMALL).rounded_md().cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, move |_, _, _| {
                    on_copy();
                })
                .child("复制"),
        )
}

/// Render a label-value info row with a copy button
pub fn render_info_row_with_copy(
    label: &str,
    value: &str,
    on_copy: impl Fn() + 'static,
) -> gpui::Div {
    div().flex().flex_row().gap_4().py_1().border_b_1().border_color(COLOR_BORDER)
        .child(div().w(gpui::px(120.0)).text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child(label.to_string()))
        .child(div().flex_1().text_size(FONT_BODY).text_color(COLOR_TEXT_BODY).child(value.to_string()))
        .child(
            div()
                .id(ElementId::Name(SharedString::from(format!("info-copy-{}", label))))
                .px_2().py_1().bg(COLOR_BG_ACTIVE).rounded_md().cursor_pointer()
                .text_size(FONT_SMALL).text_color(COLOR_TEXT_PRIMARY)
                .on_mouse_down(gpui::MouseButton::Left, move |_, _, _| {
                    on_copy();
                })
                .child("复制"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_constants() {
        assert_eq!(FONT_TITLE, gpui::px(18.0));
        assert_eq!(FONT_BODY, gpui::px(16.0));
        assert_eq!(FONT_SMALL, gpui::px(14.0));
    }

    #[test]
    fn test_status_kind_variants() {
        // Ensure all 5 variants exist and are distinguishable
        let _empty = UiStatusKind::Empty;
        let _success = UiStatusKind::Success;
        let _error = UiStatusKind::Error;
        let _warning = UiStatusKind::Warning;
        let _info = UiStatusKind::Info;
        assert_ne!(UiStatusKind::Empty, UiStatusKind::Success);
        assert_ne!(UiStatusKind::Error, UiStatusKind::Info);
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Error);
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Info);
    }

    #[test]
    fn test_warning_variant_distinct() {
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Error);
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Info);
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Success);
        assert_ne!(UiStatusKind::Warning, UiStatusKind::Empty);
    }

    #[test]
    fn test_color_warning_defined() {
        // COLOR_WARNING should be amber/yellow (0xfbbf24)
        // Just verify it compiles and is distinct from other colors
        assert_ne!(COLOR_WARNING, COLOR_ERROR);
        assert_ne!(COLOR_WARNING, COLOR_INFO);
        assert_ne!(COLOR_WARNING, COLOR_SUCCESS);
    }
}
