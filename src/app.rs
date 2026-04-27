use gpui::{div, px, rgb, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement, SharedString, Styled, Context, Window, ClipboardItem};

use crate::tabs::{CertTab, AlgoTab};
use crate::algo::{
    SymmetricAlgo, AsymmetricOp, RsaKeySize, HashAlgo, PqKemAlgo, PqSignatureAlgo,
};

/// Font size constants for consistent UI typography
const FONT_TITLE: gpui::Pixels = gpui::px(18.0);
const FONT_BODY: gpui::Pixels = gpui::px(16.0);
const FONT_SMALL: gpui::Pixels = gpui::px(14.0);

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
                weak.update(cx, |this: &mut Self, cx: &mut Context<Self>| {
                    if let DevToolsApp::Cert(t) = this {
                        t.is_importing = false;
                        cx.notify();
                    }
                }).ok();
            }
        }).detach();
    }

    /// Helper: render an input field with focus tracking via field index
    fn render_input_field(id_prefix: &str, value: &str, placeholder: &str, field_index: usize, is_focused: bool, cx: &mut Context<Self>) -> gpui::Stateful<gpui::Div> {
        let display = if value.is_empty() && !is_focused { placeholder } else { value };
        let text_color = if value.is_empty() && !is_focused { rgb(0x666677) } else { rgb(0xddddcc) };
        let border_color = if is_focused { rgb(0x3b82f6) } else { rgb(0x3a3a4a) };
        div()
            .id(ElementId::Name(SharedString::from(format!("{}-input", id_prefix))))
            .w_full()
            .px_3()
            .py_2()
            .bg(rgb(0x1e1e2e))
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .text_size(FONT_BODY)
            .text_color(text_color)
            .cursor_text()
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.algo_mut().set_focus(Some(field_index));
                cx.notify();
            }))
            .child(div().text_size(FONT_BODY).text_color(text_color).child(display.to_string()))
    }

    fn render_tab_content(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        match self {
            Self::Cert(t) => {
                let content = t.render_content();
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
            Self::Algo(t) => {
                let content = match t.active_menu {
                    0 => self.render_symmetric_tool(cx),
                    1 => self.render_asymmetric_tool(cx),
                    2 => self.render_hash_tool(cx),
                    3 => self.render_pq_kem_tool(cx),
                    4 => self.render_pq_signature_tool(cx),
                    _ => div().child("未知"),
                };
                div()
                    .flex_1()
                    .on_key_down(cx.listener(|this: &mut DevToolsApp, event: &gpui::KeyDownEvent, _window, cx| {
                        if let Some(ch) = &event.keystroke.key_char {
                            this.algo_mut().handle_key_input(ch);
                            cx.notify();
                        } else if event.keystroke.key == "backspace" {
                            this.algo_mut().handle_backspace();
                            cx.notify();
                        }
                    }))
                    .child(content)
            }
        }
    }

    fn render_symmetric_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let focused = self.algo_mut().focused_field;
        let s = &self.algo_mut().symmetric;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(rgb(0xffffff)).child("对称算法"));

        container = container.child(
            div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in SymmetricAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, {
                        let algo = *algo;
                        cx.listener(move |this, _, _, cx| {
                            this.algo_mut().symmetric.select_algo(algo);
                            cx.notify();
                        })
                    })
                    .child(format!("{}", algo)),
            );
        }

        let mode = s.mode;
        container = container.child(
            div().flex().flex_row().gap_2().mt_2().cursor_pointer()
                .child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("模式:"))
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(format!("{}", mode)))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.algo_mut().symmetric.toggle_mode();
                    cx.notify();
                })),
        );

        container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("输入数据 (十六进制):").mt_2());
        container = container.child(Self::render_input_field("sym-input", &s.input_hex, "输入十六进制数据", 0, focused == Some(0), cx));

        container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("密钥 (十六进制):").mt_2());
        container = container.child(Self::render_input_field("sym-key", &s.key_hex, &format!("输入 {} 字节密钥", s.selected_algo.key_size()), 1, focused == Some(1), cx));

        if s.selected_algo.needs_iv() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("IV (十六进制):").mt_2());
            container = container.child(Self::render_input_field("sym-iv", &s.iv_hex, "输入 16 字节 IV", 2, focused == Some(2), cx));
        }

        let need_exec = true;
        if need_exec {
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("sym-execute-btn")))
                            .px_4().py_2().bg(rgb(0x22c55e))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().symmetric.execute();
                                cx.notify();
                            }))
                            .child("执行"),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("sym-reset-btn")))
                            .px_3().py_2().bg(rgb(0x6b7280))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().symmetric.reset();
                                cx.notify();
                            }))
                            .child("重置"),
                    ),
            );
        }

        if !s.output_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("输出结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(s.output_hex.clone())));
        }

        if let Some(err) = &s.error {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    fn render_asymmetric_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let focused = self.algo_mut().focused_field;
        let a = &self.algo_mut().asymmetric;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(rgb(0xffffff)).child("非对称算法"));

        container = container.child(
            div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("操作选择:").mt_2(),
        );
        for op in [AsymmetricOp::RsaKeyGen, AsymmetricOp::RsaEncrypt, AsymmetricOp::RsaDecrypt, AsymmetricOp::EcdsaSign, AsymmetricOp::EcdsaVerify] {
            let is_active = op == a.selected_op;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, {
                        let op = op;
                        cx.listener(move |this, _, _, cx| {
                            this.algo_mut().asymmetric.select_op(op);
                            cx.notify();
                        })
                    })
                    .child(format!("{}", op)),
            );
        }

        if a.selected_op == AsymmetricOp::RsaKeyGen {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("密钥长度:").mt_2());
            for size in RsaKeySize::all() {
                let is_active = *size == a.rsa_key_size;
                let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
                container = container.child(
                    div().w(px(60.0)).px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, {
                            let size = *size;
                            cx.listener(move |this, _, _, cx| {
                                this.algo_mut().asymmetric.select_rsa_key_size(size);
                                cx.notify();
                            })
                        })
                        .child(format!("{}", size)),
                );
            }
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-execute-btn")))
                            .px_4().py_2().bg(rgb(0x22c55e))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.execute();
                                cx.notify();
                            }))
                            .child("执行"),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-reset-btn")))
                            .px_3().py_2().bg(rgb(0x6b7280))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                cx.notify();
                            }))
                            .child("重置"),
                    ),
            );
        }

        if matches!(a.selected_op, AsymmetricOp::RsaEncrypt | AsymmetricOp::RsaDecrypt) {
            let label = if a.selected_op == AsymmetricOp::RsaEncrypt { "明文输入:" } else { "密文输入 (十六进制):" };
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child(label).mt_2());
            container = container.child(Self::render_input_field("asym-input", &a.input_text, "", 3, focused == Some(3), cx));
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-execute-btn2")))
                            .px_4().py_2().bg(rgb(0x22c55e))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.execute();
                                cx.notify();
                            }))
                            .child("执行"),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-reset-btn2")))
                            .px_3().py_2().bg(rgb(0x6b7280))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                cx.notify();
                            }))
                            .child("重置"),
                    ),
            );
        }

        if matches!(a.selected_op, AsymmetricOp::EcdsaSign | AsymmetricOp::EcdsaVerify) {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("消息输入:").mt_2());
            container = container.child(Self::render_input_field("asym-msg", &a.input_text, "输入要签名的消息", 4, focused == Some(4), cx));
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-execute-btn3")))
                            .px_4().py_2().bg(rgb(0x22c55e))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.execute();
                                cx.notify();
                            }))
                            .child("执行"),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("asym-reset-btn3")))
                            .px_3().py_2().bg(rgb(0x6b7280))
                            .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                cx.notify();
                            }))
                            .child("重置"),
                    ),
            );
        }

        if !a.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(a.output_text.clone())));
        }

        if !a.rsa_pub_key_pem.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("公钥 (PEM):").mt_2());
            container = container.child(Self::copyable_display(&a.rsa_pub_key_pem, cx));
        }

        if !a.rsa_priv_key_pem.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("私钥 (PEM):").mt_2());
            container = container.child(Self::copyable_display(&a.rsa_priv_key_pem, cx));
        }

        if !a.signature_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("签名 (十六进制):").mt_2());
            container = container.child(Self::copyable_display(&a.signature_hex, cx));
        }

        if let Some(result) = a.verify_result {
            let color = if result { rgb(0x4ade80) } else { rgb(0xf87171) };
            let text = if result { "验证成功" } else { "验证失败" };
            container = container.child(div().text_size(FONT_BODY).text_color(color).child(text));
        }

        if let Some(err) = &a.error {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    fn render_hash_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let focused = self.algo_mut().focused_field;
        let h = &self.algo_mut().hash;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(rgb(0xffffff)).child("哈希算法"));

        container = container.child(
            div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in HashAlgo::all() {
            let is_active = *algo == h.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, {
                        let algo = *algo;
                        cx.listener(move |this, _, _, cx| {
                            this.algo_mut().hash.select_algo(algo);
                            cx.notify();
                        })
                    })
                    .child(format!("{} ({} 字节)", algo, algo.digest_size())),
            );
        }

        let fmt = h.input_format;
        container = container.child(
            div().flex().flex_row().gap_2().mt_2().cursor_pointer()
                .child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("输入格式:"))
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(format!("{}", fmt)))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.algo_mut().hash.toggle_format();
                    cx.notify();
                })),
        );

        let input_label = match h.input_format {
            crate::algo::hash::InputFormat::Text => "输入文本:",
            crate::algo::hash::InputFormat::Hex => "输入数据 (十六进制):",
        };
        container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child(input_label).mt_2());
        container = container.child(Self::render_input_field("hash-input", &h.input_text, "输入要计算哈希的数据", 5, focused == Some(5), cx));

        container = container.child(
            div().mt_2().flex().flex_row().gap_2()
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("hash-execute-btn")))
                        .px_4().py_2().bg(rgb(0x22c55e))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().hash.compute();
                            cx.notify();
                        }))
                        .child("执行"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("hash-reset-btn")))
                        .px_3().py_2().bg(rgb(0x6b7280))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().hash.reset();
                            cx.notify();
                        }))
                        .child("重置"),
                ),
        );

        if !h.output_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("哈希结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(h.output_hex.clone())));
        }

        if let Some(err) = &h.error {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    fn render_pq_kem_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let k = &self.algo_mut().pq_kem;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(rgb(0xffffff)).child("密码封装算法 (KEM)"));

        container = container.child(
            div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in PqKemAlgo::all() {
            let is_active = *algo == k.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, {
                        let algo = *algo;
                        cx.listener(move |this, _, _, cx| {
                            this.algo_mut().pq_kem.select_algo(algo);
                            cx.notify();
                        })
                    })
                    .child(format!("{}", algo)),
            );
        }

        container = container.child(
            div().mt_2().flex().flex_row().gap_2()
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("kem-keygen-btn")))
                        .px_4().py_2().bg(rgb(0x22c55e))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.keygen();
                            cx.notify();
                        }))
                        .child("生成密钥"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("kem-encap-btn")))
                        .px_4().py_2().bg(rgb(0x3b82f6))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.encapsulate();
                            cx.notify();
                        }))
                        .child("封装"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("kem-decap-btn")))
                        .px_4().py_2().bg(rgb(0x8b5cf6))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.decapsulate();
                            cx.notify();
                        }))
                        .child("解封装"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("kem-reset-btn")))
                        .px_3().py_2().bg(rgb(0x6b7280))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.clear();
                            cx.notify();
                        }))
                        .child("重置"),
                ),
        );

        if !k.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(k.output_text.clone()).mt_2());
        }

        if !k.public_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("公钥:").mt_2());
            container = container.child(Self::copyable_display(&k.public_key_hex, cx));
        }

        if !k.secret_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("私钥:").mt_2());
            container = container.child(Self::copyable_display(&k.secret_key_hex, cx));
        }

        if !k.ciphertext_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("密文:").mt_2());
            container = container.child(Self::copyable_display(&k.ciphertext_hex, cx));
        }

        if !k.encapsulated_secret.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("封装共享密钥:").mt_2());
            container = container.child(Self::copyable_display(&k.encapsulated_secret, cx));
        }

        if !k.decapsulated_secret.is_empty() {
            let color = if k.encapsulated_secret == k.decapsulated_secret { rgb(0x4ade80) } else { rgb(0xf87171) };
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("解封装共享密钥:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1e1e30)).rounded_md()
                .child(div().text_size(FONT_SMALL).text_color(color).child(k.decapsulated_secret.clone())));
        }

        if let Some(err) = &k.error {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    fn render_pq_signature_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let focused = self.algo_mut().focused_field;
        let s = &self.algo_mut().pq_signature;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(rgb(0xffffff)).child("数字签名算法"));

        container = container.child(
            div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in PqSignatureAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let tc = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_size(FONT_BODY).text_color(tc).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, {
                        let algo = *algo;
                        cx.listener(move |this, _, _, cx| {
                            this.algo_mut().pq_signature.select_algo(algo);
                            cx.notify();
                        })
                    })
                    .child(format!("{}", algo)),
            );
        }

        container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("消息输入:").mt_2());
        container = container.child(Self::render_input_field("pq-sig-msg", &s.input_text, "输入要签名的消息", 6, focused == Some(6), cx));

        container = container.child(
            div().mt_2().flex().flex_row().gap_2()
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("pq-sig-keygen-btn")))
                        .px_4().py_2().bg(rgb(0x22c55e))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.keygen();
                            cx.notify();
                        }))
                        .child("生成密钥"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("pq-sig-sign-btn")))
                        .px_4().py_2().bg(rgb(0x3b82f6))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.sign();
                            cx.notify();
                        }))
                        .child("签名"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("pq-sig-verify-btn")))
                        .px_4().py_2().bg(rgb(0x8b5cf6))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.verify();
                            cx.notify();
                        }))
                        .child("验证"),
                )
                .child(
                    div()
                        .id(ElementId::Name(SharedString::from("pq-sig-reset-btn")))
                        .px_3().py_2().bg(rgb(0x6b7280))
                        .text_color(rgb(0xffffff)).text_size(FONT_BODY).rounded_md().cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.clear();
                            cx.notify();
                        }))
                        .child("重置"),
                ),
        );

        if !s.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(s.output_text.clone()).mt_2());
        }

        if !s.public_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("公钥:").mt_2());
            container = container.child(Self::copyable_display(&s.public_key_hex, cx));
        }

        if !s.signature_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x888899)).child("签名:").mt_2());
            container = container.child(Self::copyable_display(&s.signature_hex, cx));
        }

        if let Some(result) = s.verify_result {
            let color = if result { rgb(0x4ade80) } else { rgb(0xf87171) };
            let text = if result { "签名验证成功" } else { "签名验证失败" };
            container = container.child(div().text_size(FONT_BODY).text_color(color).child(text));
        }

        if let Some(err) = &s.error {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    /// Helper: render a copyable display area with a copy button
    fn copyable_display(value: &str, cx: &mut Context<Self>) -> gpui::Div {
        let text = value.to_string();
        div().flex().flex_row().gap_2().items_center()
            .child(div().flex_1().px_3().py_2().bg(rgb(0x1e1e30)).rounded_md()
                .child(div().text_size(FONT_SMALL).text_color(rgb(0xddddcc)).child(value.to_string())))
            .child(
                div()
                    .id(ElementId::Name(SharedString::from(format!("copy-btn-{}", text.len()))))
                    .px_2().py_1().bg(rgb(0x3b3b5c))
                    .text_color(rgb(0xffffff)).text_size(FONT_SMALL).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _, _, cx| {
                        (**cx).write_to_clipboard(ClipboardItem::new_string(text.clone()));
                        cx.notify();
                    }))
                    .child("复制"),
            )
    }

    fn algo_mut(&mut self) -> &mut AlgoTab {
        match self {
            Self::Algo(t) => t,
            _ => unreachable!(),
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
                            .text_size(FONT_TITLE)
                            .rounded_md()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                this.select_tab(index, window, cx);
                            }))
                            .child(label)
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
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
                                    .text_size(FONT_SMALL)
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
                                    .text_size(FONT_TITLE)
                                    .rounded_md()
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                        this.select_menu(index, window, cx);
                                    }))
                                    .child(label)
                            })),
                    )
                    .child(
                        self.render_tab_content(cx),
                    ),
            )
    }
}
