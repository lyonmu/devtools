#![allow(dead_code)]
use ml_dsa::{MlDsa44, MlDsa65, MlDsa87, MlDsaParams, SigningKey, VerifyingKey, Signature, Seed, KeyGen};
use ml_dsa::signature::Keypair;
use rand::RngCore;

/// Supported post-quantum signature algorithms
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PqSignatureAlgo {
    MlDsa44,
    MlDsa65,
    MlDsa87,
}

impl std::fmt::Display for PqSignatureAlgo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PqSignatureAlgo::MlDsa44 => write!(f, "ML-DSA-44"),
            PqSignatureAlgo::MlDsa65 => write!(f, "ML-DSA-65"),
            PqSignatureAlgo::MlDsa87 => write!(f, "ML-DSA-87"),
        }
    }
}

impl PqSignatureAlgo {
    pub fn all() -> &'static [Self] {
        &[Self::MlDsa44, Self::MlDsa65, Self::MlDsa87]
    }
}

/// State for the post-quantum signature tool
pub struct PqSignatureToolState {
    pub selected_algo: PqSignatureAlgo,
    pub input_text: String,
    pub output_text: String,
    pub error: Option<String>,
    pub public_key_hex: String,
    pub secret_key_seed_hex: String,
    pub signature_hex: String,
    pub verify_result: Option<bool>,
}

impl Default for PqSignatureToolState {
    fn default() -> Self {
        Self {
            selected_algo: PqSignatureAlgo::MlDsa44,
            input_text: String::new(),
            output_text: String::new(),
            error: None,
            public_key_hex: String::new(),
            secret_key_seed_hex: String::new(),
            signature_hex: String::new(),
            verify_result: None,
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

fn reconstruct_signing_key<P: MlDsaParams>(seed_hex: &str) -> Result<SigningKey<P>, String> {
    let seed_bytes = hex_decode(seed_hex)?;
    if seed_bytes.len() != 32 {
        return Err("种子长度必须为 32 字节".to_string());
    }
    let mut seed = Seed::default();
    seed.copy_from_slice(&seed_bytes);
    Ok(P::from_seed(&seed))
}

impl PqSignatureToolState {
    pub fn keygen(&mut self) {
        self.error = None;
        let mut rng = rand::thread_rng();
        let mut seed_bytes = [0u8; 32];
        rng.fill_bytes(&mut seed_bytes);
        let mut seed = Seed::default();
        seed.copy_from_slice(&seed_bytes);

        macro_rules! do_keygen {
            ($algo:ty) => {{
                let sk: SigningKey<$algo> = <$algo>::from_seed(&seed);
                let s = sk.to_seed();
                let vk = sk.verifying_key();
                let vk_encoded = vk.encode();
                self.secret_key_seed_hex = hex_encode(&s);
                self.public_key_hex = hex_encode(vk_encoded.as_slice());
            }};
        }

        match self.selected_algo {
            PqSignatureAlgo::MlDsa44 => do_keygen!(MlDsa44),
            PqSignatureAlgo::MlDsa65 => do_keygen!(MlDsa65),
            PqSignatureAlgo::MlDsa87 => do_keygen!(MlDsa87),
        }

        self.output_text = format!("{} 密钥对生成成功", self.selected_algo);
    }

    pub fn sign(&mut self) {
        self.error = None;
        if self.input_text.is_empty() {
            self.error = Some("请输入要签名的消息".to_string());
            return;
        }
        if self.secret_key_seed_hex.is_empty() {
            self.error = Some("请先生成密钥对".to_string());
            return;
        }

        let message = self.input_text.as_bytes();

        macro_rules! do_sign {
            ($algo:ty) => {{
                match reconstruct_signing_key::<$algo>(&self.secret_key_seed_hex) {
                    Ok(sk) => {
                        match sk.signing_key().sign_deterministic(message, &[]) {
                            Ok(sig) => {
                                let encoded = sig.encode();
                                self.signature_hex = hex_encode(encoded.as_slice());
                                self.output_text = format!("签名成功 ({} 字节)", encoded.as_slice().len());
                            }
                            Err(e) => { self.error = Some(format!("签名失败: {}", e)); }
                        }
                    }
                    Err(e) => { self.error = Some(format!("私钥解析失败: {}", e)); }
                }
            }};
        }

        match self.selected_algo {
            PqSignatureAlgo::MlDsa44 => do_sign!(MlDsa44),
            PqSignatureAlgo::MlDsa65 => do_sign!(MlDsa65),
            PqSignatureAlgo::MlDsa87 => do_sign!(MlDsa87),
        }
    }

    pub fn verify(&mut self) {
        self.error = None;
        if self.public_key_hex.is_empty() {
            self.error = Some("请先生成密钥对".to_string());
            return;
        }
        if self.signature_hex.is_empty() {
            self.error = Some("请先进行签名".to_string());
            return;
        }
        if self.input_text.is_empty() {
            self.error = Some("请输入要验证的消息".to_string());
            return;
        }

        let message = self.input_text.as_bytes();
        let sig_bytes = match hex_decode(&self.signature_hex) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("签名格式错误: {}", e)); return; }
        };
        let pk_bytes = match hex_decode(&self.public_key_hex) {
            Ok(b) => b,
            Err(e) => { self.error = Some(format!("公钥格式错误: {}", e)); return; }
        };

        macro_rules! do_verify {
            ($algo:ty) => {{
                let sig = match <Signature::<$algo>>::try_from(sig_bytes.as_slice()) {
                    Ok(s) => s,
                    Err(_) => { self.error = Some("签名数据无效".to_string()); return; }
                };
                let encoded = match <ml_dsa::EncodedVerifyingKey::<$algo>>::try_from(pk_bytes.as_slice()) {
                    Ok(e) => e,
                    Err(_) => { self.error = Some("公钥数据无效".to_string()); return; }
                };
                let vk = VerifyingKey::<$algo>::decode(&encoded);
                if vk.verify_with_context(message, &[], &sig) {
                    self.verify_result = Some(true);
                    self.output_text = "签名验证成功".to_string();
                } else {
                    self.verify_result = Some(false);
                    self.output_text = "签名验证失败".to_string();
                }
            }};
        }

        match self.selected_algo {
            PqSignatureAlgo::MlDsa44 => do_verify!(MlDsa44),
            PqSignatureAlgo::MlDsa65 => do_verify!(MlDsa65),
            PqSignatureAlgo::MlDsa87 => do_verify!(MlDsa87),
        }
    }

    pub fn select_algo(&mut self, algo: PqSignatureAlgo) {
        self.selected_algo = algo;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.public_key_hex.clear();
        self.secret_key_seed_hex.clear();
        self.signature_hex.clear();
        self.verify_result = None;
        self.output_text.clear();
        self.error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_dsa44_roundtrip() {
        let mut state = PqSignatureToolState::default();
        state.keygen();
        assert!(state.error.is_none(), "keygen error: {:?}", state.error);
        assert!(!state.public_key_hex.is_empty());

        state.input_text = "Hello PQ Signature".to_string();
        state.sign();
        assert!(state.error.is_none(), "sign error: {:?}", state.error);
        assert!(!state.signature_hex.is_empty());

        state.verify();
        assert!(state.error.is_none(), "verify error: {:?}", state.error);
        assert_eq!(state.verify_result, Some(true));
    }
}

use super::registry::AlgorithmCategory;
use super::tool_trait::CryptoTool;

impl CryptoTool for PqSignatureToolState {
    fn name(&self) -> &str { "数字签名算法" }
    fn category(&self) -> AlgorithmCategory { AlgorithmCategory::Signature }
    fn execute(&mut self) { self.keygen(); }
    fn reset(&mut self) { self.clear(); }
    fn has_output(&self) -> bool {
        !self.output_text.is_empty() || !self.public_key_hex.is_empty() || !self.signature_hex.is_empty()
    }
    fn output_display(&self) -> String {
        if !self.output_text.is_empty() { self.output_text.clone() }
        else if !self.public_key_hex.is_empty() { self.public_key_hex.clone() }
        else if !self.signature_hex.is_empty() { self.signature_hex.clone() }
        else { String::new() }
    }
    fn error_display(&self) -> Option<&str> { self.error.as_deref() }
}

