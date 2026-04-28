use std::rc::Rc;

use gpui::{div, rgb, ElementId, InteractiveElement, IntoElement, ParentElement, SharedString, Styled, Window};

/// Top tab bar component — horizontal row of clickable tab buttons.
pub struct TabBar {
    tabs: Vec<SharedString>,
    active: usize,
}

impl TabBar {
    pub fn new(tabs: Vec<SharedString>, active: usize) -> Self {
        Self { tabs, active }
    }

    pub fn render(&self, on_select: Rc<dyn Fn(usize)>, _window: &mut Window) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .px_3()
            .py_2()
            .bg(rgb(0x1e1e2e))
            .children(self.tabs.iter().enumerate().map(move |(index, label)| {
                let is_active = index == self.active;
                let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                let text_color = if is_active { rgb(0xffffff) } else { rgb(0x888899) };
                let idx = index;
                let callback = Rc::clone(&on_select);
                div()
                    .id(ElementId::Name(SharedString::from(format!("tab-{idx}"))))
                    .px_3()
                    .py_1()
                    .bg(bg)
                    .text_color(text_color)
                    .text_sm()
                    .rounded_md()
                    .cursor_pointer()
                    .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, _cx| {
                        callback(idx);
                    })
                    .child(label.clone())
            }))
    }
}
