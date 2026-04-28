use gpui::{div, px, rgb, AnyElement, ClipboardItem, Context, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement, SharedString, StatefulInteractiveElement, Styled, Window};
use crate::components::input::render_text_input;
use crate::components::ui_helpers::{
    render_action_button, render_mono_output_block, render_status_banner,
    COLOR_BG_ACTIVE, COLOR_BG_MENU, COLOR_BG_PANEL, COLOR_ERROR, COLOR_INFO, COLOR_SUCCESS,
    COLOR_TEXT_BODY, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY,
    COLOR_TEXT_SECONDARY, FONT_BODY, FONT_SMALL, FONT_TITLE, UiStatusKind,
};
use crate::tabs::{AlgoInputField, AlgoTab, CertTab};
use crate::algo::{
    SymmetricAlgo, AsymmetricOp, RsaKeySize, HashAlgo, PqKemAlgo, PqSignatureAlgo,
};
/// Root application view — manages top tab bar, per-tab left menu, and right content.
pub enum DevToolsApp {
    Cert(CertTab),
    Algo(AlgoTab),
}
impl DevToolsApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
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
            0 => {
                let mut tab = CertTab::new();
                tab.error_detail_expanded = false;
                *self = Self::Cert(tab);
            },
            1 => {
                let mut tab = AlgoTab::new(cx);
                tab.error_detail_expanded = false;
                *self = Self::Algo(tab);
            },
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
    fn copy_to_clipboard_with_status(&mut self, text: String, cx: &mut Context<Self>) {
        (**cx).write_to_clipboard(ClipboardItem::new_string(text));
        match self {
            Self::Cert(tab) => tab.copy_status = Some("已复制".to_string()),
            Self::Algo(tab) => tab.copy_status = Some("已复制".to_string()),
        }
        cx.notify();
    }
    fn sync_algo_inputs_to_tool_state(&mut self, cx: &mut Context<Self>) {
        if let Self::Algo(t) = self {
            t.sync_inputs_to_tool_state(cx);
        }
    }
    fn sync_algo_tool_state_to_inputs(&mut self, cx: &mut Context<Self>) {
        if let Self::Algo(t) = self {
            t.sync_tool_state_to_inputs(cx);
        }
    }
    fn execute_focused_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(field) = (match self {
            Self::Algo(t) => t.focused_input_field(window, cx),
            _ => None,
        }) else {
            return;
        };
        self.sync_algo_inputs_to_tool_state(cx);
        match field {
            AlgoInputField::SymInput | AlgoInputField::SymKey | AlgoInputField::SymIv => {
                self.algo_mut().symmetric.execute();
            }
            _ => {}
        }
        self.sync_algo_tool_state_to_inputs(cx);
        cx.notify();
    }
    fn render_tab_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self {
            Self::Cert(t) => {
                let content = t.render_content(cx);
                if t.active_menu == 0 {
                    let cert_status = if let Some(status) = &t.copy_status {
                        render_status_banner(UiStatusKind::Success, status.clone())
                    } else if t.is_importing {
                        render_status_banner(UiStatusKind::Info, "正在解析证书...")
                    } else if let Some(err) = &t.import_error {
                        Self::render_expandable_error(
                            &format!("导入失败: {err}"),
                            err,
                            t.error_detail_expanded,
                            cx,
                        )
                    } else if let Some(cert) = &t.loaded_cert {
                        render_status_banner(UiStatusKind::Success, format!("导入成功: {}", cert.subject))
                    } else {
                        render_status_banner(UiStatusKind::Empty, "请选择证书文件")
                    };
                    return div()
                        .id(ElementId::Name(SharedString::from("cert-tab-content")))
                        .flex_1().flex().flex_col().gap_2().p_4().overflow_y_scroll()
                        .child(cert_status)
                        .child(
                            div().flex().flex_row().justify_end().py_2()
                                .child(
                                    div()
                                        .id(ElementId::Name(SharedString::from("open-file-btn")))
                                        .px_4().py_2().bg(COLOR_INFO)
                                        .text_color(COLOR_TEXT_PRIMARY).text_size(FONT_BODY).rounded_md().cursor_pointer()
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            if let DevToolsApp::Cert(_) = this {
                                                this.open_file_dialog(cx);
                                            }
                                        }))
                                        .child("选择证书文件"),
                                ),
                        )
                        .child(content)
                        .into_any_element();
                }
                let wrapper = div()
                    .id(ElementId::Name(SharedString::from("cert-tab-content")))
                    .flex_1().flex().flex_col().gap_2().p_4().overflow_y_scroll();
                let wrapper = if let Some(status) = &t.copy_status {
                    wrapper.child(render_status_banner(UiStatusKind::Success, status.clone()))
                } else {
                    wrapper
                };
                wrapper.child(content).into_any_element()
            }
            Self::Algo(t) => {
                let content = match t.active_menu {
                    0 => self.render_symmetric_tool(window, cx),
                    1 => self.render_asymmetric_tool(window, cx),
                    2 => self.render_hash_tool(window, cx),
                    3 => self.render_pq_kem_tool(cx),
                    4 => self.render_pq_signature_tool(window, cx),
                    _ => div().child("未知"),
                };
                div()
                    .id(ElementId::Name(SharedString::from("algo-tab-content")))
                    .flex_1()
                    .overflow_y_scroll()
                    .on_key_down(cx.listener(|this: &mut DevToolsApp, event: &gpui::KeyDownEvent, window, cx| {
                        if event.keystroke.key == "enter" {
                            this.execute_focused_input(window, cx);
                        }
                    }))
                    .child(content)
                    .into_any_element()
            }
        }
    }
    fn render_symmetric_tool(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let (sym_input, sym_key, sym_iv, copy_status, err_expanded) = match self {
            Self::Algo(t) => (t.sym_input.clone(), t.sym_key.clone(), t.sym_iv.clone(), t.copy_status.clone(), t.error_detail_expanded),
            _ => unreachable!(),
        };
        let s = &self.algo_mut().symmetric;
        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("对称算法"));
        container = container.child(
            div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("算法选择:").mt_2(),
        );
        for algo in SymmetricAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
            let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("模式:"))
                .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(format!("{}", mode)))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.algo_mut().symmetric.toggle_mode();
                    cx.notify();
                })),
        );
        container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("输入数据 (十六进制):").mt_2());
        container = container.child(render_text_input(sym_input, "sym-input", window, cx));
        container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("密钥 (十六进制):").mt_2());
        container = container.child(render_text_input(sym_key, "sym-key", window, cx));
        if s.selected_algo.needs_iv() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("IV (十六进制):").mt_2());
            container = container.child(render_text_input(sym_iv, "sym-iv", window, cx));
        }
        let status = if let Some(status) = copy_status {
            render_status_banner(UiStatusKind::Success, status)
        } else if let Some(err) = &s.error {
            Self::render_expandable_error(&format!("错误: {err}"), err, err_expanded, cx)
        } else if !s.output_hex.is_empty() {
            render_status_banner(UiStatusKind::Success, "执行完成")
        } else {
            render_status_banner(UiStatusKind::Empty, "请选择输入并点击执行")
        };
        container = container.child(status);
        let need_exec = true;
        if need_exec {
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        render_action_button("sym-execute-btn", "执行", COLOR_SUCCESS)
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.sync_algo_inputs_to_tool_state(cx);
                                this.algo_mut().symmetric.execute();
                                this.sync_algo_tool_state_to_inputs(cx);
                                this.algo_mut().error_detail_expanded = false;
                                cx.notify();
                            })),
                    )
                    .child(
                        render_action_button("sym-reset-btn", "重置", rgb(0x6b7280))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().symmetric.reset();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    ),
            );
        }
        if !s.output_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("输出结果:").mt_2());
            container = container.child(Self::copyable_display(&s.output_hex, cx));
        }
        if let Some(err) = &s.error {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("错误: {}", err)));
        }
        container
    }
    fn render_asymmetric_tool(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let (asym_input, copy_status, err_expanded) = match self {
            Self::Algo(t) => (t.asym_input.clone(), t.copy_status.clone(), t.error_detail_expanded),
            _ => unreachable!(),
        };
        let a = &self.algo_mut().asymmetric;
        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("非对称算法"));
        let status = if let Some(status) = copy_status {
            render_status_banner(UiStatusKind::Success, status)
        } else if let Some(err) = &a.error {
            Self::render_expandable_error(&format!("错误: {err}"), err, err_expanded, cx)
        } else if !a.output_text.is_empty() || !a.rsa_pub_key_pem.is_empty() || !a.signature_hex.is_empty() {
            render_status_banner(UiStatusKind::Success, "执行完成")
        } else {
            render_status_banner(UiStatusKind::Empty, "请选择操作并点击执行")
        };
        container = container.child(status);
        container = container.child(
            div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("操作选择:").mt_2(),
        );
        for op in [AsymmetricOp::RsaKeyGen, AsymmetricOp::RsaEncrypt, AsymmetricOp::RsaDecrypt, AsymmetricOp::EcdsaSign, AsymmetricOp::EcdsaVerify] {
            let is_active = op == a.selected_op;
            let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
            let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("密钥长度:").mt_2());
            for size in RsaKeySize::all() {
                let is_active = *size == a.rsa_key_size;
                let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
                let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
                        render_action_button("asym-execute-btn", "执行", COLOR_SUCCESS)
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.sync_algo_inputs_to_tool_state(cx);
                                this.algo_mut().asymmetric.execute();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    )
                    .child(
                        render_action_button("asym-reset-btn", "重置", rgb(0x6b7280))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    ),
            );
        }
        if matches!(a.selected_op, AsymmetricOp::RsaEncrypt | AsymmetricOp::RsaDecrypt) {
            let label = if a.selected_op == AsymmetricOp::RsaEncrypt { "明文输入:" } else { "密文输入 (十六进制):" };
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child(label).mt_2());
            container = container.child(render_text_input(asym_input.clone(), "asym-input", window, cx));
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        render_action_button("asym-execute-btn2", "执行", COLOR_SUCCESS)
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.sync_algo_inputs_to_tool_state(cx);
                                this.algo_mut().asymmetric.execute();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    )
                    .child(
                        render_action_button("asym-reset-btn2", "重置", rgb(0x6b7280))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    ),
            );
        }
        if matches!(a.selected_op, AsymmetricOp::EcdsaSign | AsymmetricOp::EcdsaVerify) {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("消息输入:").mt_2());
            container = container.child(render_text_input(asym_input.clone(), "asym-msg", window, cx));
            container = container.child(
                div().mt_2().flex().flex_row().gap_2()
                    .child(
                        render_action_button("asym-execute-btn3", "执行", COLOR_SUCCESS)
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.sync_algo_inputs_to_tool_state(cx);
                                this.algo_mut().asymmetric.execute();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    )
                    .child(
                        render_action_button("asym-reset-btn3", "重置", rgb(0x6b7280))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.algo_mut().asymmetric.reset();
                                this.algo_mut().error_detail_expanded = false;
                                this.sync_algo_tool_state_to_inputs(cx);
                                cx.notify();
                            })),
                    ),
            );
        }
        if !a.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("结果:").mt_2());
            container = container.child(Self::copyable_display(&a.output_text, cx));
        }
        if !a.rsa_pub_key_pem.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("公钥 (PEM):").mt_2());
            container = container.child(Self::copyable_display(&a.rsa_pub_key_pem, cx));
        }
        if !a.rsa_priv_key_pem.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("私钥 (PEM):").mt_2());
            container = container.child(Self::copyable_display(&a.rsa_priv_key_pem, cx));
        }
        if !a.signature_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("签名 (十六进制):").mt_2());
            container = container.child(Self::copyable_display(&a.signature_hex, cx));
        }
        if let Some(result) = a.verify_result {
            let color = if result { rgb(0x4ade80) } else { COLOR_ERROR };
            let text = if result { "验证成功" } else { "验证失败" };
            container = container.child(div().text_size(FONT_BODY).text_color(color).child(text));
        }
        if let Some(err) = &a.error {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("错误: {}", err)));
        }
        container
    }
    fn render_hash_tool(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let (hash_input, copy_status, err_expanded) = match self {
            Self::Algo(t) => (t.hash_input.clone(), t.copy_status.clone(), t.error_detail_expanded),
            _ => unreachable!(),
        };
        let h = &self.algo_mut().hash;
        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("哈希算法"));
        let status = if let Some(status) = copy_status {
            render_status_banner(UiStatusKind::Success, status)
        } else if let Some(err) = &h.error {
            Self::render_expandable_error(&format!("错误: {err}"), err, err_expanded, cx)
        } else if !h.output_hex.is_empty() {
            render_status_banner(UiStatusKind::Success, "执行完成")
        } else {
            render_status_banner(UiStatusKind::Empty, "请输入内容并点击执行")
        };
        container = container.child(status);
        container = container.child(
            div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("算法选择:").mt_2(),
        );
        for algo in HashAlgo::all() {
            let is_active = *algo == h.selected_algo;
            let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
            let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("输入格式:"))
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
        container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child(input_label).mt_2());
        container = container.child(render_text_input(hash_input, "hash-input", window, cx));
        container = container.child(
            div().mt_2().flex().flex_row().gap_2()
                .child(
                    render_action_button("hash-execute-btn", "执行", COLOR_SUCCESS)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.sync_algo_inputs_to_tool_state(cx);
                            this.algo_mut().hash.compute();
                            this.algo_mut().error_detail_expanded = false;
                            this.sync_algo_tool_state_to_inputs(cx);
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("hash-reset-btn", "重置", rgb(0x6b7280))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().hash.reset();
                            this.algo_mut().error_detail_expanded = false;
                            this.sync_algo_tool_state_to_inputs(cx);
                            cx.notify();
                        })),
                ),
        );
        if !h.output_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("哈希结果:").mt_2());
            container = container.child(Self::copyable_display(&h.output_hex, cx));
        }
        if let Some(err) = &h.error {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("错误: {}", err)));
        }
        container
    }
    fn render_pq_kem_tool(&mut self, cx: &mut Context<Self>) -> gpui::Div {
        let (copy_status, err_expanded) = match self {
            Self::Algo(t) => (t.copy_status.clone(), t.error_detail_expanded),
            _ => unreachable!(),
        };
        let k = &self.algo_mut().pq_kem;
        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("密码封装算法 (KEM)"));
        let status = if let Some(status) = copy_status {
            render_status_banner(UiStatusKind::Success, status)
        } else if let Some(err) = &k.error {
            Self::render_expandable_error(&format!("错误: {err}"), err, err_expanded, cx)
        } else if !k.output_text.is_empty() || !k.public_key_hex.is_empty() || !k.ciphertext_hex.is_empty() {
            render_status_banner(UiStatusKind::Success, "执行完成")
        } else {
            render_status_banner(UiStatusKind::Empty, "请选择算法并点击执行")
        };
        container = container.child(status);
        container = container.child(
            div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("算法选择:").mt_2(),
        );
        for algo in PqKemAlgo::all() {
            let is_active = *algo == k.selected_algo;
            let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
            let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
                    render_action_button("kem-keygen-btn", "生成密钥", COLOR_SUCCESS)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.keygen();
                            this.algo_mut().error_detail_expanded = false;
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("kem-encap-btn", "封装", COLOR_INFO)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.encapsulate();
                            this.algo_mut().error_detail_expanded = false;
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("kem-decap-btn", "解封装", rgb(0x8b5cf6))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.decapsulate();
                            this.algo_mut().error_detail_expanded = false;
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("kem-reset-btn", "重置", rgb(0x6b7280))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_kem.clear();
                            this.algo_mut().error_detail_expanded = false;
                            cx.notify();
                        })),
                ),
        );
        if !k.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(k.output_text.clone()).mt_2());
        }
        if !k.public_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("公钥:").mt_2());
            container = container.child(Self::copyable_display(&k.public_key_hex, cx));
        }
        if !k.secret_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("私钥:").mt_2());
            container = container.child(Self::copyable_display(&k.secret_key_hex, cx));
        }
        if !k.ciphertext_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("密文:").mt_2());
            container = container.child(Self::copyable_display(&k.ciphertext_hex, cx));
        }
        if !k.encapsulated_secret.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("封装共享密钥:").mt_2());
            container = container.child(Self::copyable_display(&k.encapsulated_secret, cx));
        }
        if !k.decapsulated_secret.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("解封装共享密钥:").mt_2());
            container = container.child(Self::copyable_display(&k.decapsulated_secret, cx));
        }
        if let Some(err) = &k.error {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("错误: {}", err)));
        }
        container
    }
    fn render_pq_signature_tool(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let (pq_signature_message, copy_status, err_expanded) = match self {
            Self::Algo(t) => (t.pq_signature_message.clone(), t.copy_status.clone(), t.error_detail_expanded),
            _ => unreachable!(),
        };
        let s = &self.algo_mut().pq_signature;
        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("数字签名算法"));
        let status = if let Some(status) = copy_status {
            render_status_banner(UiStatusKind::Success, status)
        } else if let Some(err) = &s.error {
            Self::render_expandable_error(&format!("错误: {err}"), err, err_expanded, cx)
        } else if !s.output_text.is_empty() || !s.public_key_hex.is_empty() || !s.signature_hex.is_empty() {
            render_status_banner(UiStatusKind::Success, "执行完成")
        } else {
            render_status_banner(UiStatusKind::Empty, "请输入消息并点击执行")
        };
        container = container.child(status);
        container = container.child(
            div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("算法选择:").mt_2(),
        );
        for algo in PqSignatureAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
            let tc = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_BODY };
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
        container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("消息输入:").mt_2());
        container = container.child(render_text_input(pq_signature_message, "pq-sig-msg", window, cx));
        container = container.child(
            div().mt_2().flex().flex_row().gap_2()
                .child(
                    render_action_button("pq-sig-keygen-btn", "生成密钥", COLOR_SUCCESS)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.keygen();
                            this.algo_mut().error_detail_expanded = false;
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("pq-sig-sign-btn", "签名", COLOR_INFO)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.sync_algo_inputs_to_tool_state(cx);
                            this.algo_mut().pq_signature.sign();
                            this.algo_mut().error_detail_expanded = false;
                            this.sync_algo_tool_state_to_inputs(cx);
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("pq-sig-verify-btn", "验证", rgb(0x8b5cf6))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.sync_algo_inputs_to_tool_state(cx);
                            this.algo_mut().pq_signature.verify();
                            this.algo_mut().error_detail_expanded = false;
                            this.sync_algo_tool_state_to_inputs(cx);
                            cx.notify();
                        })),
                )
                .child(
                    render_action_button("pq-sig-reset-btn", "重置", rgb(0x6b7280))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.algo_mut().pq_signature.clear();
                            this.algo_mut().error_detail_expanded = false;
                            this.sync_algo_tool_state_to_inputs(cx);
                            cx.notify();
                        })),
                ),
        );
        if !s.output_text.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(s.output_text.clone()).mt_2());
        }
        if !s.public_key_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("公钥:").mt_2());
            container = container.child(Self::copyable_display(&s.public_key_hex, cx));
        }
        if !s.signature_hex.is_empty() {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("签名:").mt_2());
            container = container.child(Self::copyable_display(&s.signature_hex, cx));
        }
        if let Some(result) = s.verify_result {
            let color = if result { rgb(0x4ade80) } else { COLOR_ERROR };
            let text = if result { "签名验证成功" } else { "签名验证失败" };
            container = container.child(div().text_size(FONT_BODY).text_color(color).child(text));
        }
        if let Some(err) = &s.error {
            container = container.child(div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("错误: {}", err)));
        }
        container
    }
    /// Helper: render a copyable display area with a copy button
    fn copyable_display(value: &str, cx: &mut Context<Self>) -> gpui::Div {
        let text = value.to_string();
        div().flex().flex_row().gap_2().items_center()
            .child(render_mono_output_block(value).flex_1())
            .child(
                div()
                    .id(ElementId::Name(SharedString::from(format!("copy-btn-{}", text.len()))))
                    .px_2().py_1().bg(COLOR_BG_ACTIVE)
                    .text_color(COLOR_TEXT_PRIMARY).text_size(FONT_SMALL).rounded_md().cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _, _, cx| {
                        _this.copy_to_clipboard_with_status(text.clone(), cx);
                    }))
                    .child("复制"),
            )
    }
    /// Helper: render an expandable error with summary and toggle-able detail
    fn render_expandable_error(
        summary: &str,
        detail: &str,
        expanded: bool,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let summary_text = format!("⚠ {}", summary);
        let detail_text = detail.to_string();
        let mut banner = div()
            .w_full()
            .px_3()
            .py_2()
            .bg(COLOR_BG_PANEL)
            .border_l_4()
            .border_color(COLOR_ERROR)
            .rounded_md()
            .text_size(FONT_BODY)
            .text_color(rgb(0xfecaca))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                match this {
                    Self::Cert(t) => t.error_detail_expanded = !t.error_detail_expanded,
                    Self::Algo(t) => t.error_detail_expanded = !t.error_detail_expanded,
                }
                cx.notify();
            }))
            .child(summary_text);
        if expanded {
            banner = banner.child(
                div()
                    .mt_1()
                    .text_size(FONT_SMALL)
                    .text_color(COLOR_TEXT_MUTED)
                    .child(detail_text),
            );
        }
        banner
    }
    fn algo_mut(&mut self) -> &mut AlgoTab {
        match self {
            Self::Algo(t) => t,
            _ => unreachable!(),
        }
    }
}
impl gpui::Render for DevToolsApp {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let active_tab_index = self.active_tab_index();
        let tab_names = Self::tab_names();
        let menu_items = self.menu_items();
        let active_menu = self.active_menu();
        div()
            .flex().flex_col().size_full()
            .bg(rgb(0x2a2a3a))
            .child(
                div()
                    .flex().flex_row().gap_2().px_3().py_2()
                    .bg(COLOR_BG_PANEL)
                    .children(tab_names.into_iter().enumerate().map(|(index, label)| {
                        let is_active = index == active_tab_index;
                        let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
                        let text_color = if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_SECONDARY };
                        div()
                            .id(ElementId::Name(SharedString::from(format!("tab-{}", index))))
                            .px_3().py_1()
                            .bg(bg).text_color(text_color)
                            .text_size(FONT_TITLE)
                            .rounded_md().cursor_pointer()
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
                            .w(px(200.0)).h_full()
                            .bg(COLOR_BG_MENU)
                            .p_2().gap_1().flex().flex_col()
                            .child(
                                div()
                                    .px_2().py_1()
                                    .text_size(FONT_SMALL)
                                    .text_color(COLOR_TEXT_MUTED)
                                    .child(tab_names[active_tab_index]),
                            )
                            .children(menu_items.into_iter().enumerate().map(|(index, label)| {
                                let is_active = index == active_menu;
                                let bg = if is_active { COLOR_BG_ACTIVE } else { COLOR_BG_MENU };
                                let text_color = if is_active { COLOR_TEXT_PRIMARY } else { rgb(0x9999aa) };
                                div()
                                    .id(ElementId::Name(SharedString::from(format!("menu-{}", index))))
                                    .w_full().px_3().py_2()
                                    .bg(bg)
                                    .text_color(text_color)
                                    .text_size(FONT_TITLE)
                                    .rounded_md().on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                        this.select_menu(index, window, cx);
                                    }))
                                    .child(label)
                            })),
                    )
                    .child(
                        self.render_tab_content(window, cx),
                    ),
            )
    }
}
