use gpui::{div, px, rgb, ParentElement, SharedString, Styled};

use crate::cert::ParsedCert;

/// Helper: render a label-value row
fn info_row(label: &str, value: &str) -> gpui::Div {
    div().flex().flex_row().gap_4().py_1().border_b_1().border_color(rgb(0x3a3a4a))
        .child(div().w(px(120.0)).text_sm().text_color(rgb(0x888899)).child(label.to_string()))
        .child(div().flex_1().text_sm().text_color(rgb(0xddddcc)).child(value.to_string()))
}

// ============================================================
// Certificate Tab
// ============================================================

pub struct CertTab {
    pub active_menu: usize,
    pub loaded_cert: Option<ParsedCert>,
    pub import_error: Option<String>,
    pub is_importing: bool,
}

impl CertTab {
    pub fn new() -> Self {
        Self {
            active_menu: 0,
            loaded_cert: None,
            import_error: None,
            is_importing: false,
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

    pub fn render_content(&self) -> gpui::Div {
        match self.active_menu {
            0 => self.render_import(),
            1 => self.render_basic_info(),
            2 => self.render_chain_info(),
            3 => self.render_extensions(),
            _ => div().child("未知"),
        }
    }

    fn render_import(&self) -> gpui::Div {
        let status = if self.is_importing {
            div().text_sm().text_color(rgb(0x8888aa)).child("正在解析证书...")
        } else if let Some(err) = &self.import_error {
            div().text_sm().text_color(rgb(0xf87171)).child(format!("导入失败: {err}"))
        } else if let Some(cert) = &self.loaded_cert {
            let chain_info = if !cert.chain.is_empty() {
                format!(" (证书链: {} 个证书)", cert.chain.len() + 1)
            } else {
                String::new()
            };
            div().text_sm().text_color(rgb(0x4ade80))
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
                .mt_4().p_4().bg(rgb(0x1e1e2e)).rounded_md()
                .child(div().text_sm().text_color(rgb(0xffffff)).child("证书信息预览").mb_2())
                .child(div().flex().flex_col().gap_1()
                    .child(info_row("主题", &cert.subject))
                    .child(info_row("颁发者", &cert.issuer))
                    .child(info_row("序列号", &cert.serial_number))
                    .child(info_row("有效期起始", &cert.not_before))
                    .child(info_row("有效期截止", &cert.not_after))
                    .child(info_row("签名算法", &cert.signature_algorithm))
                    .child(info_row("公钥算法", &cert.public_key_info.algorithm_name))
                    .child(info_row("密钥长度", &key_size))
                )
        } else {
            div()
        };

        div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("导入证书文件"))
            .child(div().flex().flex_col().gap_2().items_center().justify_center().flex_1()
                .child(div().text_sm().text_color(rgb(0x888899)).child("点击「选择证书文件」按钮导入文件"))
                .child(div().text_sm().text_color(rgb(0x666677)).child("支持格式: .pem, .der, .crt, .cer, .p12, .pfx"))
            )
            .child(status)
            .child(cert_preview)
    }

    fn render_basic_info(&self) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let key_size = match cert.public_key_info.key_size_bits {
                    Some(bits) => format!("{} 位", bits),
                    None => "未知".to_string(),
                };
                div()
                    .flex_1().p_4().gap_4().flex().flex_col()
                    .child(div().text_lg().text_color(rgb(0xffffff)).child("基本信息"))
                    .child(div().flex().flex_col().gap_1()
                        .child(info_row("文件路径", &cert.raw_path))
                        .child(info_row("版本", &cert.version))
                        .child(info_row("主题", &cert.subject))
                        .child(info_row("颁发者", &cert.issuer))
                        .child(info_row("序列号", &cert.serial_number))
                        .child(info_row("有效期起始", &cert.not_before))
                        .child(info_row("有效期截止", &cert.not_after))
                        .child(info_row("签名算法", &cert.signature_algorithm))
                        .child(info_row("公钥算法", &cert.public_key_info.algorithm_name))
                        .child(info_row("公钥 OID", &cert.public_key_info.algorithm_oid))
                        .child(info_row("密钥长度", &key_size))
                        .child(info_row("密钥类别", &format!("{}", cert.public_key_info.category)))
                    )
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col()
                .child(div().text_lg().text_color(rgb(0xffffff)).child("基本信息"))
                .child(div().text_sm().text_color(rgb(0x888899)).child("请先导入证书文件以查看详细信息。")),
        }
    }

    fn render_chain_info(&self) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let mut container = div()
                    .flex_1().p_4().gap_4().flex().flex_col()
                    .child(div().text_lg().text_color(rgb(0xffffff)).child("证书链信息"));

                let all_certs: Vec<&ParsedCert> = std::iter::once(cert)
                    .chain(cert.chain.iter())
                    .collect();

                if all_certs.len() == 1 {
                    container = container.child(div()
                        .text_sm().text_color(rgb(0x888899))
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
                        .child(div().text_sm().text_color(rgb(0x4ade80)).child(label))
                        .child(info_row("主题", &c.subject))
                        .child(info_row("颁发者", &c.issuer))
                        .child(info_row("有效期起始", &c.not_before))
                        .child(info_row("有效期截止", &c.not_after))
                    );
                }

                container
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col()
                .child(div().text_lg().text_color(rgb(0xffffff)).child("证书链信息"))
                .child(div().text_sm().text_color(rgb(0x888899)).child("请先导入证书文件。")),
        }
    }

    fn render_extensions(&self) -> gpui::Div {
        match &self.loaded_cert {
            Some(cert) => {
                let mut container = div()
                    .flex_1().p_4().gap_4().flex().flex_col()
                    .child(div().text_lg().text_color(rgb(0xffffff)).child("扩展项信息"));

                if cert.extensions.is_empty() {
                    container = container.child(div()
                        .text_sm().text_color(rgb(0x888899))
                        .child("无扩展项信息"));
                } else {
                    for ext in &cert.extensions {
                        let critical_label = if ext.critical { "是" } else { "否" };
                        container = container.child(div()
                            .flex().flex_col().gap_1().py_1()
                            .child(div().flex().flex_row().gap_2()
                                .child(div().text_sm().text_color(rgb(0xffffff)).child(format!("{}: ", ext.name)))
                                .child(div().text_xs().text_color(rgb(0x666677)).child(format!("OID: {}", ext.oid)))
                                .child(div().text_xs().text_color(if ext.critical { rgb(0xf87171) } else { rgb(0x888899) }).child(format!("关键: {}", critical_label)))
                            )
                            .child(div().text_sm().text_color(rgb(0xddddcc)).child(
                                div().flex().flex_col().gap_1().children(
                                    ext.value_display.lines().map(|line| div().text_sm().text_color(rgb(0xaabb99)).child(line.to_string()))
                                )
                            ))
                            .child(div().h(px(1.0)).bg(rgb(0x3a3a4a)))
                        );
                    }
                }

                container
            }
            None => div()
                .flex_1().p_4().gap_4().flex().flex_col()
                .child(div().text_lg().text_color(rgb(0xffffff)).child("扩展项信息"))
                .child(div().text_sm().text_color(rgb(0x888899)).child("请先导入证书文件。")),
        }
    }
}

// ============================================================
// Algorithm Tab (Crypto Toolkit)
// Rendering is handled in app.rs for proper event handling.
// ============================================================

use crate::algo::{
    SymmetricToolState,
    AsymmetricToolState,
    HashToolState,
    PqKemToolState,
    PqSignatureToolState,
};

pub struct AlgoTab {
    pub active_menu: usize,
    pub focused_field: Option<usize>,
    pub symmetric: SymmetricToolState,
    pub asymmetric: AsymmetricToolState,
    pub hash: HashToolState,
    pub pq_kem: PqKemToolState,
    pub pq_signature: PqSignatureToolState,
}

impl AlgoTab {
    pub fn new() -> Self {
        Self {
            active_menu: 0,
            focused_field: None,
            symmetric: SymmetricToolState::default(),
            asymmetric: AsymmetricToolState::default(),
            hash: HashToolState::default(),
            pq_kem: PqKemToolState::default(),
            pq_signature: PqSignatureToolState::default(),
        }
    }

    pub fn handle_key_input(&mut self, key: &str) {
        let focused = match self.focused_field { Some(f) => f, None => return };
        match focused {
            0 => self.symmetric.input_hex.push_str(key),
            1 => self.symmetric.key_hex.push_str(key),
            2 => self.symmetric.iv_hex.push_str(key),
            3 => self.asymmetric.input_text.push_str(key),
            4 => self.asymmetric.input_text.push_str(key),
            5 => self.hash.input_text.push_str(key),
            6 => self.pq_signature.input_text.push_str(key),
            _ => {}
        }
    }

    pub fn handle_backspace(&mut self) {
        let focused = match self.focused_field { Some(f) => f, None => return };
        match focused {
            0 => { self.symmetric.input_hex.pop(); }
            1 => { self.symmetric.key_hex.pop(); }
            2 => { self.symmetric.iv_hex.pop(); }
            3 => { self.asymmetric.input_text.pop(); }
            4 => { self.asymmetric.input_text.pop(); }
            5 => { self.hash.input_text.pop(); }
            6 => { self.pq_signature.input_text.pop(); }
            _ => {}
        }
    }

    pub fn set_focus(&mut self, field: Option<usize>) {
        self.focused_field = field;
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
