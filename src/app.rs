use gpui::{div, px, rgb, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement, SharedString, Styled, Context, Window};

use crate::tabs::{CertTab, AlgoTab};

/// Root application view — manages top tab bar, per-tab left menu, and right content.
pub enum DevToolsApp {
    Cert(CertTab),
    Algo(AlgoTab),
}

impl DevToolsApp {
    pub fn new(_cx: &mut gpui::App) -> Self {
        Self::Cert(CertTab::new())
    }

    fn active_tab_index(&self) -> usize {
        match self {
            Self::Cert(_) => 0,
            Self::Algo(_) => 1,
        }
    }

    fn tab_names() -> [&'static str; 2] {
        ["证书解析", "算法解析"]
    }

    fn menu_items(&self) -> Vec<SharedString> {
        match self {
            Self::Cert(t) => t.menu_items(),
            Self::Algo(t) => t.menu_items(),
        }
    }

    fn active_menu(&self) -> usize {
        match self {
            Self::Cert(t) => t.active_menu,
            Self::Algo(t) => t.active_menu,
        }
    }

    fn select_tab(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        match index {
            0 => *self = Self::Cert(CertTab::new()),
            1 => *self = Self::Algo(AlgoTab::new()),
            _ => {}
        }
        cx.notify();
    }

    fn select_menu(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        match self {
            Self::Cert(t) => t.active_menu = index,
            Self::Algo(t) => t.active_menu = index,
        }
        cx.notify();
    }

    /// Start async file dialog for certificate import.
    fn open_file_dialog(&mut self, cx: &mut Context<Self>) {
        // Set importing state immediately
        if let Self::Cert(t) = self {
            t.is_importing = true;
            t.import_error = None;
            cx.notify();
        }

        let weak = cx.weak_entity();
        (**cx).spawn(async move |cx| {
            let file = rfd::AsyncFileDialog::new()
                .set_title("选择证书文件")
                .add_filter("证书文件", &["pem", "der", "p12", "pfx", "cer", "crt"])
                .pick_file()
                .await;

            if let Some(file) = file {
                let path = file.path().to_path_buf();

                let result = crate::cert::detect_and_parse(&path);

                weak.update(cx, |this: &mut Self, cx: &mut Context<Self>| {
                    if let DevToolsApp::Cert(t) = this {
                        match result {
                            Ok(certs) => {
                                t.is_importing = false;
                                if let Some(first) = certs.into_iter().next() {
                                    t.loaded_cert = Some(first);
                                    t.import_error = None;
                                }
                            }
                            Err(e) => {
                                t.is_importing = false;
                                t.import_error = Some(e);
                            }
                        }
                        cx.notify();
                    }
                }).ok();
            } else {
                // User cancelled
                weak.update(cx, |this: &mut Self, cx: &mut Context<Self>| {
                    if let DevToolsApp::Cert(t) = this {
                        t.is_importing = false;
                        cx.notify();
                    }
                }).ok();
            }
        }).detach();
    }

    /// Build the content area for the current active tab.
    fn render_tab_content(&self, cx: &mut Context<Self>) -> gpui::Div {
        match self {
            Self::Cert(t) => {
                let content = t.render_content();
                // Show file import button when on the import menu
                if t.active_menu == 0 {
                    return div()
                        .flex_1().flex().flex_col()
                        .child(
                            div().flex().flex_row().justify_end().px_4().py_2()
                                .child(
                                    div()
                                        .id(ElementId::Name(SharedString::from("open-file-btn")))
                                        .px_4().py_2().bg(rgb(0x3b82f6))
                                        .text_color(rgb(0xffffff)).text_sm().rounded_md().cursor_pointer()
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            if let DevToolsApp::Cert(_) = this {
                                                this.open_file_dialog(cx);
                                            }
                                        }))
                                        .child("选择证书文件"),
                                ),
                        )
                        .child(content);
                }
                content
            }
            Self::Algo(t) => t.render_content(),
        }
    }
}

impl gpui::Render for DevToolsApp {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let active_tab_index = self.active_tab_index();
        let tab_names = Self::tab_names();
        let menu_items = self.menu_items();
        let active_menu = self.active_menu();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x2a2a3a))
            // Top tab bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .bg(rgb(0x1e1e2e))
                    .children(tab_names.into_iter().enumerate().map(|(index, label)| {
                        let is_active = index == active_tab_index;
                        let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                        let text_color = if is_active { rgb(0xffffff) } else { rgb(0x888899) };
                        div()
                            .id(ElementId::Name(SharedString::from(format!("tab-{}", index))))
                            .px_3()
                            .py_1()
                            .bg(bg)
                            .text_color(text_color)
                            .text_sm()
                            .rounded_md()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                this.select_tab(index, window, cx);
                            }))
                            .child(label)
                    })),
            )
            // Main content area: left menu + right content
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    // Left menu panel
                    .child(
                        div()
                            .w(px(200.0))
                            .h_full()
                            .bg(rgb(0x252535))
                            .p_2()
                            .gap_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .text_xs()
                                    .text_color(rgb(0x666677))
                                    .child(tab_names[active_tab_index]),
                            )
                            .children(menu_items.into_iter().enumerate().map(|(index, label)| {
                                let is_active = index == active_menu;
                                let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                                let text_color = if is_active { rgb(0xffffff) } else { rgb(0x9999aa) };
                                div()
                                    .id(ElementId::Name(SharedString::from(format!("menu-{}", index))))
                                    .w_full()
                                    .px_3()
                                    .py_2()
                                    .bg(bg)
                                    .text_color(text_color)
                                    .text_sm()
                                    .rounded_md()
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                        this.select_menu(index, window, cx);
                                    }))
                                    .child(label)
                            })),
                    )
                    // Right content panel
                    .child(
                        self.render_tab_content(cx),
                    ),
            )
    }
}
