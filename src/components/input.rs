#![allow(dead_code)]
use gpui::{div, px, rgb, ElementId, InteractiveElement, MouseButton, ParentElement, SharedString, Styled, Window, FocusHandle, Focusable, Context, KeyDownEvent};

/// Simple text input component for GPUI
pub struct TextInput {
    pub value: String,
    pub placeholder: String,
    pub focused: bool,
    focus_handle: FocusHandle,
}

impl TextInput {
    pub fn new(cx: &mut Window) -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            focused: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn focus(&mut self, cx: &mut Window) {
        self.focused = true;
        cx.focus(&self.focus_handle);
    }

    pub fn blur(&mut self) {
        self.focused = false;
    }

    pub fn handle_key(&mut self, key: &str, modifiers: &gpui::Modifiers) {
        if modifiers.platform || modifiers.control || modifiers.alt {
            // Handle Ctrl+A (select all) or Ctrl+V (paste) etc
            return;
        }
        if key == "backspace" {
            self.value.pop();
        } else if key == "escape" {
            self.blur();
        } else if key.len() == 1 {
            self.value.push_str(key);
        }
    }

    pub fn render(
        &self,
        id: &str,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let border_color = if self.focused { rgb(0x3b82f6) } else { rgb(0x3a3a4a) };
        let text_color = if self.value.is_empty() && !self.focused {
            rgb(0x666677)
        } else {
            rgb(0xddddcc)
        };
        let display_text = if self.value.is_empty() && !self.focused {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };

        div()
            .id(ElementId::Name(SharedString::from(format!("input-{}", id))))
            .w_full()
            .px_3()
            .py_2()
            .bg(rgb(0x1e1e2e))
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .text_sm()
            .text_color(text_color)
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                this.focus(window);
                cx.notify();
            }))
            .child(div().text_sm().text_color(text_color).child(display_text))
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
