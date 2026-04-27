use std::rc::Rc;

use gpui::{div, px, rgb, ElementId, InteractiveElement, IntoElement, ParentElement, SharedString, Styled, Window};

/// Left menu component — vertical column of clickable menu items.
pub struct LeftMenu {
    items: Vec<SharedString>,
    active: usize,
}

impl LeftMenu {
    pub fn new(items: Vec<SharedString>, active: usize) -> Self {
        Self { items, active }
    }

    pub fn render(&self, on_select: Rc<dyn Fn(usize)>, _window: &mut Window) -> impl IntoElement {
        div()
            .w(px(200.0))
            .h_full()
            .bg(rgb(0x252535))
            .p_2()
            .gap_1()
            .flex()
            .flex_col()
            .children(self.items.iter().enumerate().map(move |(index, label)| {
                let is_active = index == self.active;
                let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                let text_color = if is_active { rgb(0xffffff) } else { rgb(0x9999aa) };
                let idx = index;
                let callback = Rc::clone(&on_select);
                div()
                    .id(ElementId::Name(SharedString::from(format!("menu-{idx}"))))
                    .w_full()
                    .px_3()
                    .py_2()
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
