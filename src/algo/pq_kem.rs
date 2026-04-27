#![allow(dead_code)]
use ml_kem::{KemCore, MlKem512, MlKem768, MlKem1024, EncodedSizeUser};
use ml_kem::kem::{Encapsulate, Decapsulate};

/// Supported ML-KEM parameter sets
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PqKemAlgo {
    MlKem512,
    MlKem768,
    MlKem1024,
}

impl std::fmt::Display for PqKemAlgo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PqKemAlgo::MlKem512 => write!(f, "ML-KEM-512"),
            PqKemAlgo::MlKem768 => write!(f, "ML-KEM-768"),
            PqKemAlgo::MlKem1024 => write!(f, "ML-KEM-1024"),
        }
    }
}

impl PqKemAlgo {
    pub fn all() -> &'static [Self] {
        &[Self::MlKem512, Self::MlKem768, Self::MlKem1024]
    }
}

/// State for the post-quantum KEM tool
pub struct PqKemToolState {
    pub selected_algo: PqKemAlgo,
    pub output_text: String,
    pub error: Option<String>,
    pub public_key_hex: String,
    pub secret_key_hex: String,
    pub ciphertext_hex: String,
    pub encapsulated_secret: String,
    pub decapsulated_secret: String,
}

impl Default for PqKemToolState {
    fn default() -> Self {
        Self {
            selected_algo: PqKemAlgo::MlKem512,
            output_text: String::new(),
            error: None,
            public_key_hex: String::new(),
            secret_key_hex: String::new(),
            ciphertext_hex: String::new(),
            encapsulated_secret: String::new(),
            decapsulated_secret: String::new(),
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

impl PqKemToolState {
    pub fn keygen(&mut self) {
        self.error = None;
        let mut rng = rand::thread_rng();

        match self.selected_algo {
            PqKemAlgo::MlKem512 => {
                let (dk, ek) = MlKem512::generate(&mut rng);
                self.public_key_hex = hex_encode(&ek.as_bytes());
                self.secret_key_hex = hex_encode(&dk.as_bytes());
                self.output_text = "ML-KEM-512 密钥对生成成功".to_string();
            }
            PqKemAlgo::MlKem768 => {
                let (dk, ek) = MlKem768::generate(&mut rng);
                self.public_key_hex = hex_encode(&ek.as_bytes());
                self.secret_key_hex = hex_encode(&dk.as_bytes());
                self.output_text = "ML-KEM-768 密钥对生成成功".to_string();
            }
            PqKemAlgo::MlKem1024 => {
                let (dk, ek) = MlKem1024::generate(&mut rng);
                self.public_key_hex = hex_encode(&ek.as_bytes());
                self.secret_key_hex = hex_encode(&dk.as_bytes());
                self.output_text = "ML-KEM-1024 密钥对生成成功".to_string();
            }
        }
    }

    pub fn encapsulate(&mut self) {
        self.error = None;
        if self.public_key_hex.is_empty() {
            self.error = Some("请先生成密钥对".to_string());
            return;
        }
        let mut rng = rand::thread_rng();

        match self.selected_algo {
            PqKemAlgo::MlKem512 => {
                let ek = match decode_ek::<MlKem512>(&self.public_key_hex) {
                    Ok(k) => k,
                    Err(e) => { self.error = Some(e); return; }
                };
                match ek.encapsulate(&mut rng) {
                    Ok((ct, ss)) => {
                        self.ciphertext_hex = hex_encode(&ct);
                        self.encapsulated_secret = hex_encode(&ss);
                        self.output_text = "封装成功".to_string();
                    }
                    Err(_) => { self.error = Some("封装失败".to_string()); }
                }
            }
            PqKemAlgo::MlKem768 => {
                let ek = match decode_ek::<MlKem768>(&self.public_key_hex) {
                    Ok(k) => k,
                    Err(e) => { self.error = Some(e); return; }
                };
                match ek.encapsulate(&mut rng) {
                    Ok((ct, ss)) => {
                        self.ciphertext_hex = hex_encode(&ct);
                        self.encapsulated_secret = hex_encode(&ss);
                        self.output_text = "封装成功".to_string();
                    }
                    Err(_) => { self.error = Some("封装失败".to_string()); }
                }
            }
            PqKemAlgo::MlKem1024 => {
                let ek = match decode_ek::<MlKem1024>(&self.public_key_hex) {
                    Ok(k) => k,
                    Err(e) => { self.error = Some(e); return; }
                };
                match ek.encapsulate(&mut rng) {
                    Ok((ct, ss)) => {
                        self.ciphertext_hex = hex_encode(&ct);
                        self.encapsulated_secret = hex_encode(&ss);
                        self.output_text = "封装成功".to_string();
                    }
                    Err(_) => { self.error = Some("封装失败".to_string()); }
                }
            }
        }
    }

    pub fn decapsulate(&mut self) {
        self.error = None;
        if self.secret_key_hex.is_empty() {
            self.error = Some("请先生成密钥对".to_string());
            return;
        }
        if self.ciphertext_hex.is_empty() {
            self.error = Some("请先进行封装操作".to_string());
            return;
        }

        match self.selected_algo {
            PqKemAlgo::MlKem512 => {
                let (dk, ct) = match (decode_dk::<MlKem512>(&self.secret_key_hex), decode_ct::<MlKem512>(&self.ciphertext_hex)) {
                    (Ok(d), Ok(c)) => (d, c),
                    (Err(e), _) | (_, Err(e)) => { self.error = Some(e); return; }
                };
                match dk.decapsulate(&ct) {
                    Ok(ss) => {
                        self.decapsulated_secret = hex_encode(&ss);
                        if self.encapsulated_secret == self.decapsulated_secret {
                            self.output_text = "解封装成功，共享密钥匹配".to_string();
                        } else {
                            self.output_text = "解封装完成，但共享密钥不匹配!".to_string();
                        }
                    }
                    Err(_) => { self.error = Some("解封装失败".to_string()); }
                }
            }
            PqKemAlgo::MlKem768 => {
                let (dk, ct) = match (decode_dk::<MlKem768>(&self.secret_key_hex), decode_ct::<MlKem768>(&self.ciphertext_hex)) {
                    (Ok(d), Ok(c)) => (d, c),
                    (Err(e), _) | (_, Err(e)) => { self.error = Some(e); return; }
                };
                match dk.decapsulate(&ct) {
                    Ok(ss) => {
                        self.decapsulated_secret = hex_encode(&ss);
                        if self.encapsulated_secret == self.decapsulated_secret {
                            self.output_text = "解封装成功，共享密钥匹配".to_string();
                        } else {
                            self.output_text = "解封装完成，但共享密钥不匹配!".to_string();
                        }
                    }
                    Err(_) => { self.error = Some("解封装失败".to_string()); }
                }
            }
            PqKemAlgo::MlKem1024 => {
                let (dk, ct) = match (decode_dk::<MlKem1024>(&self.secret_key_hex), decode_ct::<MlKem1024>(&self.ciphertext_hex)) {
                    (Ok(d), Ok(c)) => (d, c),
                    (Err(e), _) | (_, Err(e)) => { self.error = Some(e); return; }
                };
                match dk.decapsulate(&ct) {
                    Ok(ss) => {
                        self.decapsulated_secret = hex_encode(&ss);
                        if self.encapsulated_secret == self.decapsulated_secret {
                            self.output_text = "解封装成功，共享密钥匹配".to_string();
                        } else {
                            self.output_text = "解封装完成，但共享密钥不匹配!".to_string();
                        }
                    }
                    Err(_) => { self.error = Some("解封装失败".to_string()); }
                }
            }
        }
    }

    pub fn select_algo(&mut self, algo: PqKemAlgo) {
        self.selected_algo = algo;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.public_key_hex.clear();
        self.secret_key_hex.clear();
        self.ciphertext_hex.clear();
        self.encapsulated_secret.clear();
        self.decapsulated_secret.clear();
        self.output_text.clear();
        self.error = None;
    }
}

type Kem512 = MlKem512;
type Kem768 = MlKem768;
type Kem1024 = MlKem1024;

fn decode_ek<K: KemCore>(hex: &str) -> Result<K::EncapsulationKey, String>
where
    K::EncapsulationKey: EncodedSizeUser,
{
    let bytes = hex_decode(hex)?;
    let encoded: ml_kem::Encoded<K::EncapsulationKey> =
        ml_kem::array::Array::try_from(bytes.as_slice())
            .map_err(|_| "公钥数据无效".to_string())?;
    Ok(K::EncapsulationKey::from_bytes(&encoded))
}

fn decode_dk<K: KemCore>(hex: &str) -> Result<K::DecapsulationKey, String>
where
    K::DecapsulationKey: EncodedSizeUser,
{
    let bytes = hex_decode(hex)?;
    let encoded: ml_kem::Encoded<K::DecapsulationKey> =
        ml_kem::array::Array::try_from(bytes.as_slice())
            .map_err(|_| "私钥数据无效".to_string())?;
    Ok(K::DecapsulationKey::from_bytes(&encoded))
}

fn decode_ct<K: KemCore>(hex: &str) -> Result<ml_kem::Ciphertext<K>, String> {
    let bytes = hex_decode(hex)?;
    ml_kem::array::Array::try_from(bytes.as_slice())
        .map_err(|_| "密文数据无效".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_kem512_roundtrip() {
        let mut state = PqKemToolState::default();
        state.keygen();
        assert!(state.error.is_none(), "keygen error: {:?}", state.error);
        assert!(!state.public_key_hex.is_empty());

        state.encapsulate();
        assert!(state.error.is_none(), "encapsulate error: {:?}", state.error);
        assert!(!state.ciphertext_hex.is_empty());

        state.decapsulate();
        assert!(state.error.is_none(), "decapsulate error: {:?}", state.error);
        assert!(state.output_text.contains("共享密钥匹配"));
    }
}
