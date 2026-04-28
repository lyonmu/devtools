#![allow(dead_code)]
use std::ops::Range;

use gpui::{
    div, px, relative, rgb, App, Bounds, Context, CursorStyle, Element, ElementId,
    ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId,
    InteractiveElement, IntoElement, LayoutId, MouseButton, ParentElement, Pixels, Point,
    SharedString, Style, Styled, TextRun, UTF16Selection, Window,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    SingleLine,
    MultiLine,
}

pub struct TextInputState {
    pub value: String,
    pub placeholder: String,
    pub kind: InputKind,
    pub disabled: bool,
    pub error: Option<String>,
    pub focus_handle: FocusHandle,
    pub selected_range: Range<usize>,
    pub selection_reversed: bool,
    pub marked_range: Option<Range<usize>>,
    pub last_bounds: Option<Bounds<Pixels>>,
}

impl TextInputState {
    pub fn new(
        placeholder: impl Into<String>,
        kind: InputKind,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            kind,
            disabled: false,
            error: None,
            focus_handle: cx.focus_handle(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_bounds: None,
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        let end = self.value.len();
        self.selected_range = end..end;
        self.selection_reversed = false;
        self.marked_range = None;
    }

    pub fn take_value(&self) -> String {
        self.value.clone()
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.error = None;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = clamp_to_char_boundary(&self.value, offset);
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = clamp_to_char_boundary(&self.value, offset);
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        let offset = clamp_to_char_boundary(&self.value, offset);
        self.value[..offset]
            .char_indices()
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        let offset = clamp_to_char_boundary(&self.value, offset);
        self.value[offset..]
            .char_indices()
            .nth(1)
            .map(|(idx, _)| offset + idx)
            .unwrap_or(self.value.len())
    }

    fn backspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn handle_key_down(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event.keystroke.key.as_str() {
            "backspace" => self.backspace(window, cx),
            "delete" => self.delete(window, cx),
            "left" => {
                if self.selected_range.is_empty() {
                    self.move_to(self.previous_boundary(self.cursor_offset()), cx);
                } else {
                    self.move_to(self.selected_range.start, cx);
                }
            }
            "right" => {
                if self.selected_range.is_empty() {
                    self.move_to(self.next_boundary(self.cursor_offset()), cx);
                } else {
                    self.move_to(self.selected_range.end, cx);
                }
            }
            "enter" if self.kind == InputKind::MultiLine => {
                self.replace_text_in_range(None, "\n", window, cx);
            }
            _ => {}
        }
    }
}

impl Focusable for TextInputState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextInputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = byte_range_from_utf16_range(&self.value, &range_utf16);
        adjusted_range.replace(utf16_range_from_byte_range(&self.value, &range));
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if self.disabled && !ignore_disabled_input {
            return None;
        }
        Some(UTF16Selection {
            range: utf16_range_from_byte_range(&self.value, &self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| utf16_range_from_byte_range(&self.value, range))
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.marked_range = None;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let effective_range = range_utf16.or_else(|| {
            self.marked_range
                .as_ref()
                .map(|range| utf16_range_from_byte_range(&self.value, range))
        });
        replace_text_at_utf16_range(
            &mut self.value,
            effective_range,
            text,
            &mut self.selected_range,
            &mut self.marked_range,
        );
        if self.kind == InputKind::SingleLine && self.value.contains('\n') {
            self.value = self.value.replace('\n', " ");
            let end = self.value.len();
            self.selected_range = end..end;
        }
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let effective_range = range_utf16.or_else(|| {
            self.marked_range
                .as_ref()
                .map(|range| utf16_range_from_byte_range(&self.value, range))
        });
        replace_and_mark_text_at_utf16_range(
            &mut self.value,
            effective_range,
            new_text,
            new_selected_range,
            &mut self.selected_range,
            &mut self.marked_range,
        );
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        Some(self.last_bounds.unwrap_or(element_bounds))
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        Some(utf16_len(&self.value))
    }
}

struct TextInputElement {
    input: Entity<TextInputState>,
}

impl IntoElement for TextInputElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextInputElement {
    type RequestLayoutState = ();
    type PrepaintState = String;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.input.update(cx, |input, _cx| {
            input.last_bounds = Some(bounds);
        });
        let input = self.input.read(cx);
        if input.value.is_empty() {
            input.placeholder.clone()
        } else {
            input.value.clone()
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        text: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );

        let input = self.input.read(cx);
        let style = window.text_style();
        let color = if input.value.is_empty() {
            rgb(0x666677).into()
        } else if input.disabled {
            rgb(0x777788).into()
        } else {
            rgb(0xddddcc).into()
        };
        let run = TextRun {
            len: text.len(),
            font: style.font(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window.text_system().shape_line(text.clone().into(), font_size, &[run], None);
        let _ = input;
        line.paint(bounds.origin, window.line_height(), window, cx).ok();
    }
}

pub fn render_text_input(
    input: Entity<TextInputState>,
    id: &str,
    window: &mut Window,
    cx: &mut Context<crate::app::DevToolsApp>,
) -> gpui::AnyElement {
    let input_read = input.read(cx);
    let focus_handle = input_read.focus_handle.clone();
    let is_focused = focus_handle.is_focused(window);
    let border_color = if input_read.error.is_some() {
        rgb(0xf87171)
    } else if is_focused {
        rgb(0x3b82f6)
    } else {
        rgb(0x3a3a4a)
    };
    let min_height = match input_read.kind {
        InputKind::SingleLine => px(38.0),
        InputKind::MultiLine => px(96.0),
    };
    let _ = input_read;

    let focus_input = input.clone();
    let key_input = input.clone();
    div()
        .id(ElementId::Name(SharedString::from(format!("{id}-input"))))
        .w_full()
        .min_h(min_height)
        .px_3()
        .py_2()
        .bg(rgb(0x1e1e2e))
        .border_1()
        .border_color(border_color)
        .rounded_md()
        .text_size(px(16.0))
        .line_height(px(22.0))
        .text_color(rgb(0xddddcc))
        .cursor(CursorStyle::IBeam)
        .track_focus(&focus_handle)
        .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _, window, cx| {
            focus_input.update(cx, |input, _cx| input.focus_handle.focus(window));
            cx.notify();
        }))
        .on_key_down(cx.listener(move |_this, event: &gpui::KeyDownEvent, window, cx| {
            key_input.update(cx, |input, input_cx| input.handle_key_down(event, window, input_cx));
            cx.notify();
        }))
        .child(TextInputElement { input })
        .into_any_element()
}

fn utf16_len(value: &str) -> usize {
    value.chars().map(char::len_utf16).sum()
}

fn byte_offset_from_utf16(value: &str, utf16_offset: usize) -> usize {
    let mut byte_offset = 0;
    let mut utf16_count = 0;
    for ch in value.chars() {
        if utf16_count >= utf16_offset {
            break;
        }
        utf16_count += ch.len_utf16();
        byte_offset += ch.len_utf8();
    }
    clamp_to_char_boundary(value, byte_offset)
}

fn utf16_offset_from_byte(value: &str, byte_offset: usize) -> usize {
    let byte_offset = clamp_to_char_boundary(value, byte_offset);
    value[..byte_offset].chars().map(char::len_utf16).sum()
}

fn byte_range_from_utf16_range(value: &str, range: &Range<usize>) -> Range<usize> {
    byte_offset_from_utf16(value, range.start)..byte_offset_from_utf16(value, range.end)
}

fn utf16_range_from_byte_range(value: &str, range: &Range<usize>) -> Range<usize> {
    utf16_offset_from_byte(value, range.start)..utf16_offset_from_byte(value, range.end)
}

fn clamp_to_char_boundary(value: &str, offset: usize) -> usize {
    let mut offset = offset.min(value.len());
    while offset > 0 && !value.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn replace_text_at_utf16_range(
    value: &mut String,
    range_utf16: Option<Range<usize>>,
    text: &str,
    selection: &mut Range<usize>,
    marked_range: &mut Option<Range<usize>>,
) {
    let range = range_utf16
        .as_ref()
        .map(|range| byte_range_from_utf16_range(value, range))
        .or_else(|| marked_range.clone())
        .unwrap_or_else(|| selection.clone());
    let start = clamp_to_char_boundary(value, range.start);
    let end = clamp_to_char_boundary(value, range.end).max(start);
    value.replace_range(start..end, text);
    let cursor = start + text.len();
    *selection = cursor..cursor;
    *marked_range = None;
}

fn replace_and_mark_text_at_utf16_range(
    value: &mut String,
    range_utf16: Option<Range<usize>>,
    text: &str,
    new_selected_range_utf16: Option<Range<usize>>,
    selection: &mut Range<usize>,
    marked_range: &mut Option<Range<usize>>,
) {
    let range = range_utf16
        .as_ref()
        .map(|range| byte_range_from_utf16_range(value, range))
        .or_else(|| marked_range.clone())
        .unwrap_or_else(|| selection.clone());
    let start = clamp_to_char_boundary(value, range.start);
    let end = clamp_to_char_boundary(value, range.end).max(start);
    value.replace_range(start..end, text);
    let marked_end = start + text.len();
    *marked_range = if text.is_empty() { None } else { Some(start..marked_end) };
    *selection = new_selected_range_utf16
        .as_ref()
        .map(|relative| {
            let rel_start = byte_offset_from_utf16(text, relative.start);
            let rel_end = byte_offset_from_utf16(text, relative.end);
            start + rel_start..start + rel_end
        })
        .unwrap_or(marked_end..marked_end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_replacement_uses_utf16_range_and_clears_marked_text() {
        let mut value = "A😀中Z".to_string();
        let mut selection = 0..0;
        let mut marked = Some(1..5);

        replace_text_at_utf16_range(&mut value, Some(1..3), "B", &mut selection, &mut marked);

        assert_eq!(value, "AB中Z");
        assert_eq!(selection, 2..2);
        assert_eq!(marked, None);
    }

    #[test]
    fn marked_replacement_tracks_preedit_range_and_relative_selection() {
        let mut value = "hello".to_string();
        let mut selection = 5..5;
        let mut marked = None;

        replace_and_mark_text_at_utf16_range(
            &mut value,
            None,
            "中国",
            Some(1..2),
            &mut selection,
            &mut marked,
        );

        assert_eq!(value, "hello中国");
        assert_eq!(marked, Some(5..11));
        assert_eq!(selection, 8..11);
    }
}
