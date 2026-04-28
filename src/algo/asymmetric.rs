#![allow(dead_code)]
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, traits::PublicKeyParts};
use rsa::pkcs8::{LineEnding, DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use p256::ecdsa::{self, SigningKey, VerifyingKey};
use p256::SecretKey;
use rsa::signature::{Signer, Verifier};

/// Supported asymmetric operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AsymmetricOp {
    RsaKeyGen,
    RsaEncrypt,
    RsaDecrypt,
    EcdsaSign,
    EcdsaVerify,
}

impl std::fmt::Display for AsymmetricOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsymmetricOp::RsaKeyGen => write!(f, "RSA 密钥生成"),
            AsymmetricOp::RsaEncrypt => write!(f, "RSA 加密"),
            AsymmetricOp::RsaDecrypt => write!(f, "RSA 解密"),
            AsymmetricOp::EcdsaSign => write!(f, "ECDSA 签名"),
            AsymmetricOp::EcdsaVerify => write!(f, "ECDSA 验证"),
        }
    }
}

/// RSA key sizes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RsaKeySize {
    B2048,
    B3072,
    B4096,
}

impl std::fmt::Display for RsaKeySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsaKeySize::B2048 => write!(f, "2048"),
            RsaKeySize::B3072 => write!(f, "3072"),
            RsaKeySize::B4096 => write!(f, "4096"),
        }
    }
}

impl RsaKeySize {
    pub fn bits(&self) -> usize {
        match self {
            RsaKeySize::B2048 => 2048,
            RsaKeySize::B3072 => 3072,
            RsaKeySize::B4096 => 4096,
        }
    }
    pub fn all() -> &'static [Self] {
        &[Self::B2048, Self::B3072, Self::B4096]
    }
}

/// State for the asymmetric crypto tool
pub struct AsymmetricToolState {
    pub selected_op: AsymmetricOp,
    pub input_text: String,
    pub output_text: String,
    pub error: Option<String>,
    pub rsa_key_size: RsaKeySize,
    pub rsa_pub_key_pem: String,
    pub rsa_priv_key_pem: String,
    pub signature_hex: String,
    pub verify_result: Option<bool>,
    pub ecc_pub_key_hex: String,
    pub ecc_priv_key_hex: String,
}

impl Default for AsymmetricToolState {
    fn default() -> Self {
        Self {
            selected_op: AsymmetricOp::RsaKeyGen,
            input_text: String::new(),
            output_text: String::new(),
            error: None,
            rsa_key_size: RsaKeySize::B2048,
            rsa_pub_key_pem: String::new(),
            rsa_priv_key_pem: String::new(),
            signature_hex: String::new(),
            verify_result: None,
            ecc_pub_key_hex: String::new(),
            ecc_priv_key_hex: String::new(),
        }
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = hex.chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    if cleaned.len() % 2 != 0 {
        return Err("十六进制字符串长度必须为偶数".to_string());
    }
    (0..cleaned.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|_| format!("无效的十六进制字符: {}", &cleaned[i..i + 2]))
        })
        .collect()
}

impl AsymmetricToolState {
    pub fn execute(&mut self) {
        self.error = None;
        self.output_text.clear();
        self.verify_result = None;
        match self.selected_op {
            AsymmetricOp::RsaKeyGen => self.rsa_keygen(),
            AsymmetricOp::RsaEncrypt => self.rsa_encrypt(),
            AsymmetricOp::RsaDecrypt => self.rsa_decrypt(),
            AsymmetricOp::EcdsaSign => self.ecdsa_sign(),
            AsymmetricOp::EcdsaVerify => self.ecdsa_verify(),
        }
    }

    fn rsa_keygen(&mut self) {
        let mut rng = OsRng;
        let bits = self.rsa_key_size.bits();
        match RsaPrivateKey::new(&mut rng, bits) {
            Ok(private_key) => {
                let public_key = RsaPublicKey::from(&private_key);
                match public_key.to_public_key_pem(LineEnding::LF) {
                    Ok(pem) => self.rsa_pub_key_pem = pem,
                    Err(e) => self.rsa_pub_key_pem = format!("导出公钥失败: {}", e),
                }
                match private_key.to_pkcs8_pem(LineEnding::LF) {
                    Ok(pem) => self.rsa_priv_key_pem = pem.to_string(),
                    Err(e) => self.rsa_priv_key_pem = format!("导出私钥失败: {}", e),
                }
                self.output_text = format!("RSA-{} 密钥对生成成功", self.rsa_key_size);
            }
            Err(e) => { self.error = Some(format!("密钥生成失败: {}", e)); }
        }
    }

    fn rsa_encrypt(&mut self) {
        if self.rsa_pub_key_pem.is_empty() {
            self.error = Some("请先生成 RSA 密钥对或导入公钥".to_string());
            return;
        }
        let public_key = match RsaPublicKey::from_public_key_pem(&self.rsa_pub_key_pem) {
            Ok(k) => k,
            Err(e) => { self.error = Some(format!("解析公钥失败: {}", e)); return; }
        };
        let mut rng = OsRng;
        let plaintext = self.input_text.as_bytes();
        if plaintext.len() > public_key.size() - 11 {
            self.error = Some(format!("明文过长，RSA-{} 最多加密 {} 字节", public_key.n().bits(), public_key.size() - 11));
            return;
        }
        match public_key.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext) {
            Ok(ciphertext) => { self.output_text = hex_encode(&ciphertext); }
            Err(e) => { self.error = Some(format!("加密失败: {}", e)); }
        }
    }

    fn rsa_decrypt(&mut self) {
        if self.rsa_priv_key_pem.is_empty() {
            self.error = Some("请先生成 RSA 密钥对或导入私钥".to_string());
            return;
        }
        let private_key = match RsaPrivateKey::from_pkcs8_pem(&self.rsa_priv_key_pem) {
            Ok(k) => k,
            Err(e) => { self.error = Some(format!("解析私钥失败: {}", e)); return; }
        };
        let ciphertext = match hex_decode(&self.input_text) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("密文格式错误: {}", e)); return; }
        };
        match private_key.decrypt(Pkcs1v15Encrypt, &ciphertext) {
            Ok(plaintext) => { self.output_text = String::from_utf8_lossy(&plaintext).to_string(); }
            Err(e) => { self.error = Some(format!("解密失败: {}", e)); }
        }
    }

    fn ecdsa_keygen(&mut self) {
        let secret_key = SecretKey::random(&mut OsRng);
        self.ecc_priv_key_hex = hex_encode(secret_key.to_bytes().as_ref());
        let public_key = secret_key.public_key();
        self.ecc_pub_key_hex = hex_encode(public_key.to_sec1_bytes().as_ref());
        self.output_text = "P-256 ECDSA 密钥对生成成功".to_string();
    }

    fn ecdsa_sign(&mut self) {
        if self.ecc_priv_key_hex.is_empty() {
            self.ecdsa_keygen();
        }
        let priv_bytes = match hex_decode(&self.ecc_priv_key_hex) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("私钥格式错误: {}", e)); return; }
        };
        let secret_key = match SecretKey::from_slice(&priv_bytes) {
            Ok(k) => k,
            Err(e) => { self.error = Some(format!("私钥解析失败: {}", e)); return; }
        };
        let signing_key = SigningKey::from(&secret_key);
        let message = if self.input_text.is_empty() {
            b"default message for signing"
        } else {
            self.input_text.as_bytes()
        };
        let signature: ecdsa::Signature = signing_key.sign(message);
        self.signature_hex = hex_encode(signature.to_bytes().as_ref());
        self.output_text = format!("签名成功 ({} 字节)", signature.to_bytes().len());
    }

    fn ecdsa_verify(&mut self) {
        if self.ecc_pub_key_hex.is_empty() {
            self.error = Some("请先生成密钥对".to_string());
            return;
        }
        if self.signature_hex.is_empty() {
            self.error = Some("请先进行签名".to_string());
            return;
        }
        let pub_bytes = match hex_decode(&self.ecc_pub_key_hex) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("公钥格式错误: {}", e)); return; }
        };
        let verifying_key = match VerifyingKey::from_sec1_bytes(&pub_bytes) {
            Ok(k) => k,
            Err(e) => { self.error = Some(format!("公钥解析失败: {}", e)); return; }
        };
        let sig_bytes = match hex_decode(&self.signature_hex) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("签名格式错误: {}", e)); return; }
        };
        let signature = match ecdsa::Signature::try_from(sig_bytes.as_slice()) {
            Ok(s) => s,
            Err(_) => { self.error = Some("签名数据无效".to_string()); return; }
        };
        let message = if self.input_text.is_empty() {
            b"default message for signing"
        } else {
            self.input_text.as_bytes()
        };
        if verifying_key.verify(message, &signature).is_ok() {
            self.verify_result = Some(true);
            self.output_text = "签名验证成功".to_string();
        } else {
            self.verify_result = Some(false);
            self.output_text = "签名验证失败".to_string();
        }
    }

    pub fn select_op(&mut self, op: AsymmetricOp) {
        self.selected_op = op;
        self.output_text.clear();
        self.error = None;
        self.verify_result = None;
    }

    pub fn select_rsa_key_size(&mut self, size: RsaKeySize) {
        self.rsa_key_size = size;
        self.rsa_pub_key_pem.clear();
        self.rsa_priv_key_pem.clear();
        self.output_text.clear();
        self.error = None;
    }

    pub fn reset(&mut self) {
        self.input_text.clear();
        self.output_text.clear();
        self.error = None;
        self.rsa_pub_key_pem.clear();
        self.rsa_priv_key_pem.clear();
        self.signature_hex.clear();
        self.verify_result = None;
        self.ecc_pub_key_hex.clear();
        self.ecc_priv_key_hex.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_rsa_state() -> AsymmetricToolState {
        let mut state = AsymmetricToolState::default();
        state.selected_op = AsymmetricOp::RsaKeyGen;
        state.execute();
        assert!(state.error.is_none(), "keygen error: {:?}", state.error);
        state
    }

    fn clone_state_for_test(state: &AsymmetricToolState) -> AsymmetricToolState {
        AsymmetricToolState {
            selected_op: state.selected_op,
            input_text: state.input_text.clone(),
            output_text: state.output_text.clone(),
            error: state.error.clone(),
            rsa_key_size: state.rsa_key_size,
            rsa_pub_key_pem: state.rsa_pub_key_pem.clone(),
            rsa_priv_key_pem: state.rsa_priv_key_pem.clone(),
            signature_hex: state.signature_hex.clone(),
            verify_result: state.verify_result,
            ecc_pub_key_hex: state.ecc_pub_key_hex.clone(),
            ecc_priv_key_hex: state.ecc_priv_key_hex.clone(),
        }
    }

    #[test]
    fn test_rsa_encrypt_decrypt_roundtrip() {
        let mut state = AsymmetricToolState::default();
        state.rsa_keygen();
        assert!(state.rsa_pub_key_pem.contains("PUBLIC KEY"));
        assert!(state.rsa_priv_key_pem.contains("PRIVATE KEY"));

        state.selected_op = AsymmetricOp::RsaEncrypt;
        state.input_text = "Hello RSA".to_string();
        state.execute();
        assert!(state.error.is_none(), "encrypt error: {:?}", state.error);
        let ciphertext = state.output_text.clone();

        state.selected_op = AsymmetricOp::RsaDecrypt;
        state.input_text = ciphertext;
        state.execute();
        assert!(state.error.is_none(), "decrypt error: {:?}", state.error);
        assert_eq!(state.output_text, "Hello RSA");
    }

    #[test]
    fn rsa_keygen_and_key_size_selection_update_state() {
        let mut state = generated_rsa_state();

        assert!(state.rsa_pub_key_pem.contains("PUBLIC KEY"));
        assert!(state.rsa_priv_key_pem.contains("PRIVATE KEY"));
        assert!(state.output_text.contains("RSA-2048 密钥对生成成功"));

        state.select_rsa_key_size(RsaKeySize::B3072);
        assert_eq!(state.rsa_key_size, RsaKeySize::B3072);
        assert!(state.rsa_pub_key_pem.is_empty());
        assert!(state.rsa_priv_key_pem.is_empty());
        assert!(state.output_text.is_empty());
        assert!(state.error.is_none());
    }

    #[test]
    fn rsa_encrypt_reports_missing_invalid_and_oversized_input_errors() {
        let mut missing = AsymmetricToolState::default();
        missing.selected_op = AsymmetricOp::RsaEncrypt;
        missing.input_text = "hello".to_string();
        missing.execute();
        assert_eq!(missing.error.as_deref(), Some("请先生成 RSA 密钥对或导入公钥"));
        assert!(missing.output_text.is_empty());

        let mut invalid_pem = AsymmetricToolState::default();
        invalid_pem.selected_op = AsymmetricOp::RsaEncrypt;
        invalid_pem.rsa_pub_key_pem = "not a public key".to_string();
        invalid_pem.input_text = "hello".to_string();
        invalid_pem.execute();
        assert!(invalid_pem.error.as_deref().unwrap_or_default().contains("解析公钥失败"));

        let mut oversized = generated_rsa_state();
        oversized.selected_op = AsymmetricOp::RsaEncrypt;
        oversized.input_text = "x".repeat(300);
        oversized.execute();
        assert!(oversized.error.as_deref().unwrap_or_default().contains("明文过长"));
        assert!(oversized.output_text.is_empty());
    }

    #[test]
    fn rsa_decrypt_reports_missing_invalid_hex_and_invalid_private_key_errors() {
        let mut missing = AsymmetricToolState::default();
        missing.selected_op = AsymmetricOp::RsaDecrypt;
        missing.input_text = "00".to_string();
        missing.execute();
        assert_eq!(missing.error.as_deref(), Some("请先生成 RSA 密钥对或导入私钥"));

        let mut invalid_hex = generated_rsa_state();
        invalid_hex.selected_op = AsymmetricOp::RsaDecrypt;
        invalid_hex.input_text = "abc".to_string();
        invalid_hex.execute();
        assert!(invalid_hex.error.as_deref().unwrap_or_default().contains("密文格式错误"));

        let mut invalid_pem = AsymmetricToolState::default();
        invalid_pem.selected_op = AsymmetricOp::RsaDecrypt;
        invalid_pem.rsa_priv_key_pem = "not a private key".to_string();
        invalid_pem.input_text = "00".to_string();
        invalid_pem.execute();
        assert!(invalid_pem.error.as_deref().unwrap_or_default().contains("解析私钥失败"));
    }

    #[test]
    fn test_ecdsa_sign_verify_roundtrip() {
        let mut state = AsymmetricToolState::default();
        state.selected_op = AsymmetricOp::EcdsaSign;
        state.input_text = "Hello ECDSA".to_string();
        state.execute();
        assert!(state.error.is_none(), "sign error: {:?}", state.error);
        assert!(!state.signature_hex.is_empty());

        state.selected_op = AsymmetricOp::EcdsaVerify;
        state.execute();
        assert!(state.error.is_none(), "verify error: {:?}", state.error);
        assert_eq!(state.verify_result, Some(true));
    }

    #[test]
    fn ecdsa_sign_generates_keys_and_verify_detects_tampering() {
        let mut state = AsymmetricToolState::default();
        state.selected_op = AsymmetricOp::EcdsaSign;
        state.input_text = "signed message".to_string();
        state.execute();

        assert!(state.error.is_none(), "sign error: {:?}", state.error);
        assert!(!state.ecc_pub_key_hex.is_empty());
        assert!(!state.ecc_priv_key_hex.is_empty());
        assert!(!state.signature_hex.is_empty());

        state.selected_op = AsymmetricOp::EcdsaVerify;
        state.execute();
        assert_eq!(state.verify_result, Some(true));

        state.input_text = "tampered message".to_string();
        state.execute();
        assert!(state.error.is_none(), "verify error: {:?}", state.error);
        assert_eq!(state.verify_result, Some(false));
        assert_eq!(state.output_text, "签名验证失败");
    }

    #[test]
    fn ecdsa_verify_reports_missing_and_malformed_inputs() {
        let mut signed = AsymmetricToolState::default();
        signed.selected_op = AsymmetricOp::EcdsaSign;
        signed.input_text = "message".to_string();
        signed.execute();
        assert!(signed.error.is_none());

        let mut missing_pub = clone_state_for_test(&signed);
        missing_pub.selected_op = AsymmetricOp::EcdsaVerify;
        missing_pub.ecc_pub_key_hex.clear();
        missing_pub.execute();
        assert_eq!(missing_pub.error.as_deref(), Some("请先生成密钥对"));

        let mut missing_sig = clone_state_for_test(&signed);
        missing_sig.selected_op = AsymmetricOp::EcdsaVerify;
        missing_sig.signature_hex.clear();
        missing_sig.execute();
        assert_eq!(missing_sig.error.as_deref(), Some("请先进行签名"));

        let mut invalid_pub_hex = clone_state_for_test(&signed);
        invalid_pub_hex.selected_op = AsymmetricOp::EcdsaVerify;
        invalid_pub_hex.ecc_pub_key_hex = "zz".to_string();
        invalid_pub_hex.execute();
        assert!(invalid_pub_hex.error.as_deref().unwrap_or_default().contains("公钥格式错误"));

        let mut invalid_pub_bytes = clone_state_for_test(&signed);
        invalid_pub_bytes.selected_op = AsymmetricOp::EcdsaVerify;
        invalid_pub_bytes.ecc_pub_key_hex = "00".to_string();
        invalid_pub_bytes.execute();
        assert!(invalid_pub_bytes.error.as_deref().unwrap_or_default().contains("公钥解析失败"));

        let mut invalid_sig_hex = clone_state_for_test(&signed);
        invalid_sig_hex.selected_op = AsymmetricOp::EcdsaVerify;
        invalid_sig_hex.signature_hex = "zz".to_string();
        invalid_sig_hex.execute();
        assert!(invalid_sig_hex.error.as_deref().unwrap_or_default().contains("签名格式错误"));

        let mut invalid_sig_bytes = clone_state_for_test(&signed);
        invalid_sig_bytes.selected_op = AsymmetricOp::EcdsaVerify;
        invalid_sig_bytes.signature_hex = "00".to_string();
        invalid_sig_bytes.execute();
        assert_eq!(invalid_sig_bytes.error.as_deref(), Some("签名数据无效"));
    }

    #[test]
    fn select_op_clears_transient_output_without_clearing_keys() {
        let mut state = AsymmetricToolState::default();
        state.selected_op = AsymmetricOp::EcdsaSign;
        state.input_text = "message".to_string();
        state.execute();
        let pub_key = state.ecc_pub_key_hex.clone();
        let priv_key = state.ecc_priv_key_hex.clone();
        state.output_text = "old".to_string();
        state.error = Some("错误".to_string());
        state.verify_result = Some(false);

        state.select_op(AsymmetricOp::EcdsaVerify);

        assert_eq!(state.selected_op, AsymmetricOp::EcdsaVerify);
        assert!(state.output_text.is_empty());
        assert!(state.error.is_none());
        assert_eq!(state.verify_result, None);
        assert_eq!(state.ecc_pub_key_hex, pub_key);
        assert_eq!(state.ecc_priv_key_hex, priv_key);
    }

    #[test]
    fn reset_clears_outputs_and_preserves_operation_and_key_size() {
        let mut state = AsymmetricToolState {
            selected_op: AsymmetricOp::EcdsaVerify,
            input_text: "消息".to_string(),
            output_text: "结果".to_string(),
            error: Some("错误".to_string()),
            rsa_key_size: RsaKeySize::B3072,
            rsa_pub_key_pem: "public".to_string(),
            rsa_priv_key_pem: "private".to_string(),
            signature_hex: "abcd".to_string(),
            verify_result: Some(true),
            ecc_pub_key_hex: "01".to_string(),
            ecc_priv_key_hex: "02".to_string(),
        };

        state.reset();

        assert_eq!(state.selected_op, AsymmetricOp::EcdsaVerify);
        assert_eq!(state.rsa_key_size, RsaKeySize::B3072);
        assert!(state.input_text.is_empty());
        assert!(state.output_text.is_empty());
        assert!(state.error.is_none());
        assert!(state.rsa_pub_key_pem.is_empty());
        assert!(state.rsa_priv_key_pem.is_empty());
        assert!(state.signature_hex.is_empty());
        assert_eq!(state.verify_result, None);
        assert!(state.ecc_pub_key_hex.is_empty());
        assert!(state.ecc_priv_key_hex.is_empty());
    }
}

use super::registry::AlgorithmCategory;
use super::tool_trait::CryptoTool;

impl CryptoTool for AsymmetricToolState {
    fn name(&self) -> &str { "非对称算法" }
    fn category(&self) -> AlgorithmCategory { AlgorithmCategory::Asymmetric }
    fn execute(&mut self) { AsymmetricToolState::execute(self); }
    fn reset(&mut self) { AsymmetricToolState::reset(self); }
    fn has_output(&self) -> bool {
        !self.output_text.is_empty() || !self.rsa_pub_key_pem.is_empty() || !self.signature_hex.is_empty()
    }
    fn output_display(&self) -> String {
        if !self.output_text.is_empty() { self.output_text.clone() }
        else if !self.rsa_pub_key_pem.is_empty() { self.rsa_pub_key_pem.clone() }
        else if !self.signature_hex.is_empty() { self.signature_hex.clone() }
        else { String::new() }
    }
    fn error_display(&self) -> Option<&str> { self.error.as_deref() }
}
