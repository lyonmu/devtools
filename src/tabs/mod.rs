use gpui::{div, px, rgb, AppContext, Entity, ElementId, InteractiveElement, MouseButton, ParentElement, SharedString, Styled};
use crate::cert::ParsedCert;
use crate::components::ui_helpers::{
    render_info_row, render_mono_output_block, COLOR_BG_ACTIVE, COLOR_BG_PANEL, COLOR_BORDER,
    COLOR_ERROR, COLOR_TEXT_BODY, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY, COLOR_TEXT_SECONDARY,
    FONT_BODY, FONT_SMALL, FONT_TITLE,
};
fn copyable_render_info_row(label: &str, value: &str, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
    let text = value.to_string();
    div().flex().flex_row().gap_4().py_1().border_b_1().border_color(COLOR_BORDER)
        .child(div().w(px(120.0)).text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child(label.to_string()))
        .child(div().flex_1().text_size(FONT_BODY).text_color(COLOR_TEXT_BODY).child(value.to_string()))
        .child(div()
            .id(ElementId::Name(SharedString::from(format!("cert-copy-{label}"))))
            .px_2().py_1().bg(COLOR_BG_ACTIVE).rounded_md().cursor_pointer()
            .text_size(FONT_SMALL).text_color(COLOR_TEXT_PRIMARY)
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                if let crate::app::DevToolsApp::Cert(tab) = this {
                    (**cx).write_to_clipboard(gpui::ClipboardItem::new_string(text.clone()));
                    tab.copy_status = Some("已复制".to_string());
                    cx.notify();
                }
            }))
            .child("复制"))
}
// ============================================================
// Certificate Tab
// ============================================================
pub struct CertTab {
    pub active_menu: usize,
    pub loaded_cert: Option<ParsedCert>,
    pub import_error: Option<String>,
    pub is_importing: bool,
    pub copy_status: Option<String>,
    pub error_detail_expanded: bool,
}
impl CertTab {
    pub fn new() -> Self {
        Self {
            active_menu: 0,
            loaded_cert: None,
            import_error: None,
            is_importing: false,
            copy_status: None,
            error_detail_expanded: false,
        }
    }
    pub fn menu_items(&self) -> Vec<SharedString> {
        vec![
            "文件导入".into(),
            "基本信息".into(),
            "证书链信息".into(),
            "扩展项信息".into(),
        ]
    }
    pub fn render_content(&self, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
        match self.active_menu {
            0 => self.render_import(cx),
            1 => self.render_basic_info(cx),
            2 => self.render_chain_info(cx),
            3 => self.render_extensions(cx),
            _ => div().child("未知"),
        }
    }
    fn render_import(&self, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
        let status = if self.is_importing {
            div().text_size(FONT_BODY).text_color(rgb(0x8888aa)).child("正在解析证书...")
        } else if let Some(err) = &self.import_error {
            div().text_size(FONT_BODY).text_color(COLOR_ERROR).child(format!("导入失败: {err}"))
        } else if let Some(cert) = &self.loaded_cert {
            let chain_info = if !cert.chain.is_empty() {
                format!(" (证书链: {} 个证书)", cert.chain.len() + 1)
            } else {
                String::new()
            };
            div().text_size(FONT_BODY).text_color(rgb(0x4ade80))
                .child(format!("导入成功: {}{}", cert.subject, chain_info))
        } else {
            div()
        };
        let cert_preview = if let Some(cert) = &self.loaded_cert {
            let key_size = match cert.public_key_info.key_size_bits {
                Some(bits) => format!("{} 位", bits),
                None => "未知".to_string(),
            };
            div()
                .mt_4().p_4().bg(COLOR_BG_PANEL).rounded_md()
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_PRIMARY).child("证书信息预览").mb_2())
                .child(div().flex().flex_col().gap_1()
                    .child(copyable_render_info_row("主题", &cert.subject, cx))
                    .child(copyable_render_info_row("颁发者", &cert.issuer, cx))
                    .child(copyable_render_info_row("序列号", &cert.serial_number, cx))
                    .child(copyable_render_info_row("有效期起始", &cert.not_before, cx))
                    .child(copyable_render_info_row("有效期截止", &cert.not_after, cx))
                    .child(copyable_render_info_row("签名算法", &cert.signature_algorithm, cx))
                    .child(copyable_render_info_row("公钥算法", &cert.public_key_info.algorithm_name, cx))
                    .child(render_info_row("密钥长度", &key_size))
                )
        } else {
            div()
        };
        div()
            .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("导入证书文件"))
            .child(div().flex().flex_col().gap_2().items_center().justify_center().flex_1()
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("点击「选择证书文件」按钮导入文件"))
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_MUTED).child("支持格式: .pem, .der, .crt, .cer, .p12, .pfx"))
            )
            .child(status)
            .child(cert_preview)
    }
    fn render_basic_info(&self, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let key_size = match cert.public_key_info.key_size_bits {
                    Some(bits) => format!("{} 位", bits),
                    None => "未知".to_string(),
                };
                div()
                    .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("基本信息"))
                    .child(div().flex().flex_col().gap_1()
                        .child(render_info_row("文件路径", &cert.raw_path))
                        .child(render_info_row("版本", &cert.version))
                        .child(copyable_render_info_row("主题", &cert.subject, cx))
                        .child(copyable_render_info_row("颁发者", &cert.issuer, cx))
                        .child(copyable_render_info_row("序列号", &cert.serial_number, cx))
                        .child(copyable_render_info_row("有效期起始", &cert.not_before, cx))
                        .child(copyable_render_info_row("有效期截止", &cert.not_after, cx))
                        .child(copyable_render_info_row("签名算法", &cert.signature_algorithm, cx))
                        .child(copyable_render_info_row("公钥算法", &cert.public_key_info.algorithm_name, cx))
                        .child(copyable_render_info_row("公钥 OID", &cert.public_key_info.algorithm_oid, cx))
                        .child(render_info_row("密钥长度", &key_size))
                        .child(render_info_row("密钥类别", &format!("{}", cert.public_key_info.category)))
                    )
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("基本信息"))
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("请先导入证书文件以查看详细信息。")),
        }
    }
    fn render_chain_info(&self, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let mut container = div()
                    .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("证书链信息"));
                let all_certs: Vec<&ParsedCert> = std::iter::once(cert)
                    .chain(cert.chain.iter())
                    .collect();
                if all_certs.len() == 1 {
                    container = container.child(div()
                        .text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY)
                        .child("此证书为独立证书，无证书链"));
                }
                for (i, c) in all_certs.iter().enumerate() {
                    let label = match i {
                        0 => "叶证书 (当前)",
                        n if n == all_certs.len() - 1 && all_certs.len() > 1 => "根证书",
                        _ => "中间证书",
                    };
                    container = container.child(div()
                        .flex().flex_col().gap_1().mt_2()
                        .child(div().text_size(FONT_BODY).text_color(rgb(0x4ade80)).child(label))
                        .child(copyable_render_info_row("主题", &c.subject, cx))
                        .child(copyable_render_info_row("颁发者", &c.issuer, cx))
                        .child(copyable_render_info_row("有效期起始", &c.not_before, cx))
                        .child(copyable_render_info_row("有效期截止", &c.not_after, cx))
                    );
                }
                container
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("证书链信息"))
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("请先导入证书文件。")),
        }
    }
    fn render_extensions(&self, cx: &mut gpui::Context<crate::app::DevToolsApp>) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let mut container = div()
                    .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("扩展项信息"));
                if cert.extensions.is_empty() {
                    container = container.child(div()
                        .text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY)
                        .child("无扩展项信息"));
                } else {
                    for ext in &cert.extensions {
                        let critical_label = if ext.critical { "是" } else { "否" };
                        container = container.child(div()
                            .flex().flex_col().gap_1().py_1()
                            .child(div().flex().flex_row().gap_2()
                                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_PRIMARY).child(format!("{}: ", ext.name)))
                                .child(div().text_size(FONT_SMALL).text_color(COLOR_TEXT_MUTED).child(format!("OID: {}", ext.oid)))
                                .child(div().text_size(FONT_SMALL).text_color(if ext.critical { COLOR_ERROR } else { COLOR_TEXT_SECONDARY }).child(format!("关键: {}", critical_label)))
                            )
                            .child(copyable_render_info_row("扩展值", &ext.value_display, cx))
                            .child(render_mono_output_block(&ext.value_display))
                            .child(div().h(px(1.0)).bg(COLOR_BORDER))
                        );
                    }
                }
                container
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col().child(div().text_size(FONT_TITLE).text_color(COLOR_TEXT_PRIMARY).child("扩展项信息"))
                .child(div().text_size(FONT_BODY).text_color(COLOR_TEXT_SECONDARY).child("请先导入证书文件。")),
        }
    }
}
// ============================================================
// Algorithm Tab (Crypto Toolkit)
// Rendering is handled in app.rs for proper event handling.
// ============================================================
use crate::algo::{
    AsymmetricToolState,
    HashToolState,
    PqKemToolState,
    PqSignatureToolState,
    SymmetricToolState,
};
use crate::components::input::{InputKind, TextInputState};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlgoInputField {
    SymInput,
    SymKey,
    SymIv,
    AsymInput,
    HashInput,
    PqSignatureMessage,
}
pub struct AlgoTab {
    pub active_menu: usize,
    pub symmetric: SymmetricToolState,
    pub asymmetric: AsymmetricToolState,
    pub hash: HashToolState,
    pub pq_kem: PqKemToolState,
    pub pq_signature: PqSignatureToolState,
    pub copy_status: Option<String>,
    pub sym_input: Entity<TextInputState>,
    pub sym_key: Entity<TextInputState>,
    pub sym_iv: Entity<TextInputState>,
    pub asym_input: Entity<TextInputState>,
    pub hash_input: Entity<TextInputState>,
    pub pq_signature_message: Entity<TextInputState>,
    pub error_detail_expanded: bool,
}
impl AlgoTab {
    pub fn new(cx: &mut gpui::Context<crate::app::DevToolsApp>) -> Self {
        Self {
            active_menu: 0,
            symmetric: SymmetricToolState::default(),
            asymmetric: AsymmetricToolState::default(),
            hash: HashToolState::default(),
            pq_kem: PqKemToolState::default(),
            pq_signature: PqSignatureToolState::default(),
            copy_status: None,
            sym_input: cx.new(|cx| TextInputState::new("输入十六进制数据", InputKind::SingleLine, cx)),
            sym_key: cx.new(|cx| TextInputState::new("输入密钥", InputKind::SingleLine, cx)),
            sym_iv: cx.new(|cx| TextInputState::new("输入 16 字节 IV", InputKind::SingleLine, cx)),
            asym_input: cx.new(|cx| TextInputState::new("输入文本或密文", InputKind::MultiLine, cx)),
            hash_input: cx.new(|cx| TextInputState::new("输入要计算哈希的数据", InputKind::MultiLine, cx)),
            pq_signature_message: cx.new(|cx| TextInputState::new("输入要签名的消息", InputKind::MultiLine, cx)),
            error_detail_expanded: false,
        }
    }
    pub fn input_for_field(&self, field: AlgoInputField) -> Entity<TextInputState> {
        match field {
            AlgoInputField::SymInput => self.sym_input.clone(),
            AlgoInputField::SymKey => self.sym_key.clone(),
            AlgoInputField::SymIv => self.sym_iv.clone(),
            AlgoInputField::AsymInput => self.asym_input.clone(),
            AlgoInputField::HashInput => self.hash_input.clone(),
            AlgoInputField::PqSignatureMessage => self.pq_signature_message.clone(),
        }
    }
    pub fn sync_inputs_to_tool_state(&mut self, cx: &mut gpui::App) {
        self.symmetric.input_hex = self.sym_input.read(cx).take_value();
        self.symmetric.key_hex = self.sym_key.read(cx).take_value();
        self.symmetric.iv_hex = self.sym_iv.read(cx).take_value();
        self.asymmetric.input_text = self.asym_input.read(cx).take_value();
        self.hash.input_text = self.hash_input.read(cx).take_value();
        self.pq_signature.input_text = self.pq_signature_message.read(cx).take_value();
    }
    pub fn sync_tool_state_to_inputs(&mut self, cx: &mut gpui::App) {
        self.sym_input.update(cx, |input, _| {
            input.set_value(self.symmetric.input_hex.clone());
            input.set_error(self.symmetric.error.clone());
        });
        self.sym_key.update(cx, |input, _| {
            input.set_value(self.symmetric.key_hex.clone());
            input.set_error(self.symmetric.error.clone());
        });
        self.sym_iv.update(cx, |input, _| {
            input.set_value(self.symmetric.iv_hex.clone());
            input.set_error(self.symmetric.error.clone());
        });
        self.asym_input.update(cx, |input, _| {
            input.set_value(self.asymmetric.input_text.clone());
            input.set_error(self.asymmetric.error.clone());
        });
        self.hash_input.update(cx, |input, _| {
            input.set_value(self.hash.input_text.clone());
            input.set_error(self.hash.error.clone());
        });
        self.pq_signature_message.update(cx, |input, _| {
            input.set_value(self.pq_signature.input_text.clone());
            input.set_error(self.pq_signature.error.clone());
        });
    }
    pub fn focused_input_field(&self, window: &gpui::Window, cx: &gpui::App) -> Option<AlgoInputField> {
        [
            AlgoInputField::SymInput,
            AlgoInputField::SymKey,
            AlgoInputField::SymIv,
            AlgoInputField::AsymInput,
            AlgoInputField::HashInput,
            AlgoInputField::PqSignatureMessage,
        ]
        .into_iter()
        .find(|field| self.input_for_field(*field).read(cx).focus_handle.is_focused(window))
    }
    pub fn menu_items(&self) -> Vec<SharedString> {
        vec![
            "对称算法".into(),
            "非对称算法".into(),
            "哈希算法".into(),
            "密码封装算法".into(),
            "数字签名算法".into(),
        ]
    }
}
