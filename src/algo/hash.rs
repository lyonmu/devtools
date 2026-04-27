#![allow(dead_code)]
use sha2::{Sha256, Sha384, Sha512, Digest};
use sm3::Sm3 as Sm3Hasher;

/// Supported hash algorithms
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HashAlgo {
    Sha256,
    Sha384,
    Sha512,
    Sm3,
}

impl std::fmt::Display for HashAlgo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgo::Sha256 => write!(f, "SHA-256"),
            HashAlgo::Sha384 => write!(f, "SHA-384"),
            HashAlgo::Sha512 => write!(f, "SHA-512"),
            HashAlgo::Sm3 => write!(f, "SM3"),
        }
    }
}

impl HashAlgo {
    pub fn digest_size(&self) -> usize {
        match self {
            HashAlgo::Sha256 => 32,
            HashAlgo::Sha384 => 48,
            HashAlgo::Sha512 => 64,
            HashAlgo::Sm3 => 32,
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Sha256, Self::Sha384, Self::Sha512, Self::Sm3]
    }
}

/// State for the hash computation tool
pub struct HashToolState {
    pub selected_algo: HashAlgo,
    pub input_text: String,
    pub input_format: InputFormat,
    pub output_hex: String,
    pub error: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputFormat {
    Text,
    Hex,
}

impl std::fmt::Display for InputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputFormat::Text => write!(f, "文本"),
            InputFormat::Hex => write!(f, "十六进制"),
        }
    }
}

impl Default for HashToolState {
    fn default() -> Self {
        Self {
            selected_algo: HashAlgo::Sha256,
            input_text: String::new(),
            input_format: InputFormat::Text,
            output_hex: String::new(),
            error: None,
        }
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = hex.chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    if cleaned.is_empty() {
        return Ok(Vec::new());
    }
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

impl HashToolState {
    pub fn compute(&mut self) {
        self.error = None;
        self.output_hex.clear();

        let input_bytes = match self.input_format {
            InputFormat::Text => self.input_text.as_bytes().to_vec(),
            InputFormat::Hex => match hex_decode(&self.input_text) {
                Ok(b) => b,
                Err(e) => {
                    self.error = Some(format!("输入格式错误: {}", e));
                    return;
                }
            },
        };

        let digest: Vec<u8> = match self.selected_algo {
            HashAlgo::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(&input_bytes);
                hasher.finalize().to_vec()
            }
            HashAlgo::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(&input_bytes);
                hasher.finalize().to_vec()
            }
            HashAlgo::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(&input_bytes);
                hasher.finalize().to_vec()
            }
            HashAlgo::Sm3 => {
                let mut hasher = Sm3Hasher::new();
                hasher.update(&input_bytes);
                hasher.finalize().to_vec()
            }
        };

        self.output_hex = hex_encode(&digest);
    }

    pub fn select_algo(&mut self, algo: HashAlgo) {
        self.selected_algo = algo;
        self.output_hex.clear();
        self.error = None;
    }

    pub fn toggle_format(&mut self) {
        self.input_format = match self.input_format {
            InputFormat::Text => InputFormat::Hex,
            InputFormat::Hex => InputFormat::Text,
        };
        self.output_hex.clear();
        self.error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let mut state = HashToolState::default();
        state.input_text = "Hello, World!".to_string();
        state.compute();
        assert!(state.error.is_none(), "hash error: {:?}", state.error);
        assert_eq!(state.output_hex.len(), 64);
    }

    #[test]
    fn test_sm3_hash() {
        let mut state = HashToolState::default();
        state.select_algo(HashAlgo::Sm3);
        state.input_text = "测试数据".to_string();
        state.compute();
        assert!(state.error.is_none(), "hash error: {:?}", state.error);
        assert_eq!(state.output_hex.len(), 64);
    }

    #[test]
    fn test_hex_input_format() {
        let mut state = HashToolState::default();
        state.input_format = InputFormat::Hex;
        state.input_text = "48656c6c6f".to_string();
        state.compute();
        assert!(state.error.is_none());
        assert!(!state.output_hex.is_empty());
    }

    #[test]
    fn test_digest_sizes() {
        let mut state = HashToolState::default();
        state.input_text = "test".to_string();

        state.select_algo(HashAlgo::Sha256);
        state.compute();
        assert_eq!(state.output_hex.len(), 64); // 32 bytes = 64 hex chars

        state.select_algo(HashAlgo::Sha384);
        state.compute();
        assert_eq!(state.output_hex.len(), 96); // 48 bytes = 96 hex chars

        state.select_algo(HashAlgo::Sha512);
        state.compute();
        assert_eq!(state.output_hex.len(), 128); // 64 bytes = 128 hex chars
    }
}
