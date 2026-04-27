use gpui::{div, px, rgb, ElementId, InteractiveElement, ParentElement, SharedString, Styled};

use crate::cert::ParsedCert;
use crate::algo::{
    SymmetricAlgo, SymmetricToolState,
    AsymmetricOp, AsymmetricToolState, RsaKeySize,
    HashAlgo, HashToolState,
    PqKemAlgo, PqKemToolState,
    PqSignatureAlgo, PqSignatureToolState,
};

/// Helper: render a label-value row
fn info_row(label: &str, value: &str) -> gpui::Div {
    div().flex().flex_row().gap_4().py_1().border_b_1().border_color(rgb(0x3a3a4a))
        .child(div().w(px(120.0)).text_sm().text_color(rgb(0x888899)).child(label.to_string()))
        .child(div().flex_1().text_sm().text_color(rgb(0xddddcc)).child(value.to_string()))
}

/// Helper: render a text input div
fn text_input_div(id_prefix: &str, value: &str, placeholder: &str) -> gpui::Stateful<gpui::Div> {
    let display = if value.is_empty() { placeholder } else { value };
    let text_color = if value.is_empty() { rgb(0x666677) } else { rgb(0xddddcc) };
    div()
        .id(ElementId::Name(SharedString::from(format!("{}-input", id_prefix))))
        .w_full()
        .px_3()
        .py_2()
        .bg(rgb(0x1e1e2e))
        .border_1()
        .border_color(rgb(0x3a3a4a))
        .rounded_md()
        .text_sm()
        .text_color(text_color)
        .child(div().text_sm().text_color(text_color).child(display.to_string()))
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

        div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("导入证书文件"))
            .child(div().flex().flex_col().gap_2().items_center().justify_center().flex_1()
                .child(div().text_sm().text_color(rgb(0x888899)).child("点击「选择证书文件」按钮导入文件"))
                .child(div().text_sm().text_color(rgb(0x666677)).child("支持格式: .pem, .der, .crt, .cer, .p12, .pfx"))
            )
            .child(status)
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
//
// Menu: 对称算法 | 非对称算法 | 哈希算法 | 密码封装算法 | 数字签名算法
// ============================================================

pub struct AlgoTab {
    pub active_menu: usize,
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
            symmetric: SymmetricToolState::default(),
            asymmetric: AsymmetricToolState::default(),
            hash: HashToolState::default(),
            pq_kem: PqKemToolState::default(),
            pq_signature: PqSignatureToolState::default(),
        }
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

    pub fn render_content(&self) -> gpui::Div {
        match self.active_menu {
            0 => self.render_symmetric_tool(),
            1 => self.render_asymmetric_tool(),
            2 => self.render_hash_tool(),
            3 => self.render_pq_kem_tool(),
            4 => self.render_pq_signature_tool(),
            _ => div().child("未知"),
        }
    }

    // -------------------------------------------------------
    // 对称算法 — AES (ECB/CBC), SM4 (ECB/CBC)
    // -------------------------------------------------------

    fn render_symmetric_tool(&self) -> gpui::Div {
        let s = &self.symmetric;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("对称算法"));

        container = container.child(
            div().text_sm().text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in SymmetricAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                    .child(format!("{}", algo)),
            );
        }

        container = container.child(
            div().flex().flex_row().gap_2().mt_2()
                .child(div().text_sm().text_color(rgb(0x888899)).child("模式:"))
                .child(div().text_sm().text_color(rgb(0x4ade80)).child(format!("{}", s.mode))),
        );

        container = container.child(div().text_sm().text_color(rgb(0x888899)).child("输入数据 (十六进制):").mt_2());
        container = container.child(text_input_div("sym-input", &s.input_hex, "输入十六进制数据"));

        container = container.child(div().text_sm().text_color(rgb(0x888899)).child("密钥 (十六进制):").mt_2());
        container = container.child(text_input_div("sym-key", &s.key_hex, &format!("输入 {} 字节密钥", s.selected_algo.key_size())));

        if s.selected_algo.needs_iv() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("IV (十六进制):").mt_2());
            container = container.child(text_input_div("sym-iv", &s.iv_hex, "输入 16 字节 IV"));
        }

        if !s.output_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("输出结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_sm().text_color(rgb(0x4ade80)).child(s.output_hex.clone())));
        }

        if let Some(err) = &s.error {
            container = container.child(div().text_sm().text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    // -------------------------------------------------------
    // 非对称算法 — RSA (keygen/encrypt/decrypt), ECDSA (sign/verify)
    // -------------------------------------------------------

    fn render_asymmetric_tool(&self) -> gpui::Div {
        let a = &self.asymmetric;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("非对称算法"));

        container = container.child(
            div().text_sm().text_color(rgb(0x888899)).child("操作选择:").mt_2(),
        );
        for op in [AsymmetricOp::RsaKeyGen, AsymmetricOp::RsaEncrypt, AsymmetricOp::RsaDecrypt, AsymmetricOp::EcdsaSign, AsymmetricOp::EcdsaVerify] {
            let is_active = op == a.selected_op;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                    .child(format!("{}", op)),
            );
        }

        if a.selected_op == AsymmetricOp::RsaKeyGen {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("密钥长度:").mt_2());
            for size in RsaKeySize::all() {
                let is_active = *size == a.rsa_key_size;
                let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
                let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
                container = container.child(
                    div().w(px(60.0)).px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                        .child(format!("{}", size)),
                );
            }
        }

        if matches!(a.selected_op, AsymmetricOp::RsaEncrypt | AsymmetricOp::RsaDecrypt) {
            let label = if a.selected_op == AsymmetricOp::RsaEncrypt {
                "明文输入:"
            } else {
                "密文输入 (十六进制):"
            };
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child(label).mt_2());
            container = container.child(text_input_div("asym-input", &a.input_text, ""));
        }

        if matches!(a.selected_op, AsymmetricOp::EcdsaSign | AsymmetricOp::EcdsaVerify) {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("消息输入:").mt_2());
            container = container.child(text_input_div("asym-msg", &a.input_text, "输入要签名的消息"));
        }

        if !a.output_text.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_sm().text_color(rgb(0x4ade80)).child(a.output_text.clone())));
        }

        if !a.rsa_pub_key_pem.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("公钥 (PEM):").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(a.rsa_pub_key_pem.clone())));
        }

        if !a.rsa_priv_key_pem.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("私钥 (PEM):").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(a.rsa_priv_key_pem.clone())));
        }

        if !a.signature_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("签名 (十六进制):").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_sm().text_color(rgb(0xddddcc)).child(a.signature_hex.clone())));
        }

        if let Some(result) = a.verify_result {
            let color = if result { rgb(0x4ade80) } else { rgb(0xf87171) };
            let text = if result { "验证成功" } else { "验证失败" };
            container = container.child(div().text_sm().text_color(color).child(text));
        }

        if let Some(err) = &a.error {
            container = container.child(div().text_sm().text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    // -------------------------------------------------------
    // 哈希算法 — SHA-256/384/512, SM3
    // -------------------------------------------------------

    fn render_hash_tool(&self) -> gpui::Div {
        let h = &self.hash;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("哈希算法"));

        container = container.child(
            div().text_sm().text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in HashAlgo::all() {
            let is_active = *algo == h.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                    .child(format!("{} ({} 字节)", algo, algo.digest_size())),
            );
        }

        container = container.child(
            div().flex().flex_row().gap_2().mt_2()
                .child(div().text_sm().text_color(rgb(0x888899)).child("输入格式:"))
                .child(div().text_sm().text_color(rgb(0x4ade80)).child(format!("{}", h.input_format))),
        );

        let input_label = match h.input_format {
            crate::algo::hash::InputFormat::Text => "输入文本:",
            crate::algo::hash::InputFormat::Hex => "输入数据 (十六进制):",
        };
        container = container.child(div().text_sm().text_color(rgb(0x888899)).child(input_label).mt_2());
        container = container.child(text_input_div("hash-input", &h.input_text, "输入要计算哈希的数据"));

        if !h.output_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("哈希结果:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_sm().text_color(rgb(0x4ade80)).child(h.output_hex.clone())));
        }

        if let Some(err) = &h.error {
            container = container.child(div().text_sm().text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    // -------------------------------------------------------
    // 密码封装算法 (KEM) — ML-KEM-512/768/1024
    // -------------------------------------------------------

    fn render_pq_kem_tool(&self) -> gpui::Div {
        let k = &self.pq_kem;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("密码封装算法 (KEM)"));

        container = container.child(
            div().text_sm().text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in PqKemAlgo::all() {
            let is_active = *algo == k.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                    .child(format!("{}", algo)),
            );
        }

        if !k.output_text.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x4ade80)).child(k.output_text.clone()).mt_2());
        }

        if !k.public_key_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("公钥:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(k.public_key_hex.clone())));
        }

        if !k.ciphertext_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("密文:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(k.ciphertext_hex.clone())));
        }

        if !k.encapsulated_secret.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("封装共享密钥:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(k.encapsulated_secret.clone())));
        }

        if !k.decapsulated_secret.is_empty() {
            let color = if k.encapsulated_secret == k.decapsulated_secret { rgb(0x4ade80) } else { rgb(0xf87171) };
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("解封装共享密钥:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(color).child(k.decapsulated_secret.clone())));
        }

        if let Some(err) = &k.error {
            container = container.child(div().text_sm().text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }

    // -------------------------------------------------------
    // 数字签名算法 — ML-DSA-44/65/87
    // -------------------------------------------------------

    fn render_pq_signature_tool(&self) -> gpui::Div {
        let s = &self.pq_signature;

        let mut container = div()
            .flex_1().p_4().gap_4().flex().flex_col()
            .child(div().text_lg().text_color(rgb(0xffffff)).child("数字签名算法"));

        container = container.child(
            div().text_sm().text_color(rgb(0x888899)).child("算法选择:").mt_2(),
        );
        for algo in PqSignatureAlgo::all() {
            let is_active = *algo == s.selected_algo;
            let bg = if is_active { rgb(0x3b3b5c) } else { rgb(0x252535) };
            let text_color = if is_active { rgb(0xffffff) } else { rgb(0xddddcc) };
            container = container.child(
                div().w_full().px_2().py_1().bg(bg).text_sm().text_color(text_color).rounded_md()
                    .child(format!("{}", algo)),
            );
        }

        container = container.child(div().text_sm().text_color(rgb(0x888899)).child("消息输入:").mt_2());
        container = container.child(text_input_div("pq-sig-msg", &s.input_text, "输入要签名的消息"));

        if !s.output_text.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x4ade80)).child(s.output_text.clone()).mt_2());
        }

        if !s.public_key_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("公钥:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_xs().text_color(rgb(0xddddcc)).child(s.public_key_hex.clone())));
        }

        if !s.signature_hex.is_empty() {
            container = container.child(div().text_sm().text_color(rgb(0x888899)).child("签名:").mt_2());
            container = container.child(div().px_3().py_2().bg(rgb(0x1a1a2a)).rounded_md()
                .child(div().text_sm().text_color(rgb(0xddddcc)).child(s.signature_hex.clone())));
        }

        if let Some(result) = s.verify_result {
            let color = if result { rgb(0x4ade80) } else { rgb(0xf87171) };
            let text = if result { "签名验证成功" } else { "签名验证失败" };
            container = container.child(div().text_sm().text_color(color).child(text));
        }

        if let Some(err) = &s.error {
            container = container.child(div().text_sm().text_color(rgb(0xf87171)).child(format!("错误: {}", err)));
        }

        container
    }
}
