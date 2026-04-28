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

    pub fn reset(&mut self) {
        self.input_text.clear();
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

    fn compute_hex(algo: HashAlgo, input: &str, format: InputFormat) -> HashToolState {
        let mut state = HashToolState::default();
        state.select_algo(algo);
        state.input_text = input.to_string();
        state.input_format = format;
        state.compute();
        state
    }

    #[test]
    fn known_answer_vectors_for_abc_match_published_digests() {
        let cases = [
            (
                HashAlgo::Sha256,
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                HashAlgo::Sha384,
                concat!(
                    "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded163",
                    "1a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
                ),
            ),
            (
                HashAlgo::Sha512,
                concat!(
                    "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea2",
                    "0a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd",
                    "454d4423643ce80e2a9ac94fa54ca49f"
                ),
            ),
            (
                HashAlgo::Sm3,
                concat!(
                    "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2",
                    "297da02b8f4ba8e0"
                ),
            ),
        ];

        for (algo, expected) in cases {
            let state = compute_hex(algo, "abc", InputFormat::Text);
            assert!(state.error.is_none(), "{algo:?} error: {:?}", state.error);
            assert_eq!(state.output_hex, expected);
        }
    }

    #[test]
    fn text_and_hex_input_produce_identical_sha256_output() {
        let text_state = compute_hex(HashAlgo::Sha256, "abc", InputFormat::Text);
        let hex_state = compute_hex(HashAlgo::Sha256, "616263", InputFormat::Hex);

        assert!(text_state.error.is_none());
        assert!(hex_state.error.is_none());
        assert_eq!(text_state.output_hex, hex_state.output_hex);
    }

    #[test]
    fn invalid_hex_input_sets_error_and_leaves_output_empty() {
        for input in ["abc", "zz"] {
            let state = compute_hex(HashAlgo::Sha256, input, InputFormat::Hex);

            assert!(state.output_hex.is_empty());
            assert!(
                state.error.as_deref().unwrap_or_default().contains("输入格式错误"),
                "unexpected error for {input}: {:?}",
                state.error
            );
        }
    }

    #[test]
    fn empty_text_hashes_to_sha256_empty_string_digest() {
        let state = compute_hex(HashAlgo::Sha256, "", InputFormat::Text);

        assert!(state.error.is_none());
        assert_eq!(
            state.output_hex,
            concat!(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934",
                "ca495991b7852b855"
            )
        );
    }

    #[test]
    fn changing_algorithm_or_format_clears_output_and_error() {
        let mut state = HashToolState {
            selected_algo: HashAlgo::Sha256,
            input_text: "abc".to_string(),
            input_format: InputFormat::Text,
            output_hex: "digest".to_string(),
            error: Some("错误".to_string()),
        };

        state.select_algo(HashAlgo::Sha512);
        assert_eq!(state.selected_algo, HashAlgo::Sha512);
        assert!(state.output_hex.is_empty());
        assert!(state.error.is_none());

        state.output_hex = "digest".to_string();
        state.error = Some("错误".to_string());
        state.toggle_format();
        assert_eq!(state.input_format, InputFormat::Hex);
        assert!(state.output_hex.is_empty());
        assert!(state.error.is_none());
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

    #[test]
    fn reset_clears_input_output_and_preserves_algorithm_and_format() {
        let mut state = HashToolState {
            selected_algo: HashAlgo::Sm3,
            input_text: "48656c6c6f".to_string(),
            input_format: InputFormat::Hex,
            output_hex: "abcd".to_string(),
            error: Some("错误".to_string()),
        };

        state.reset();

        assert_eq!(state.selected_algo, HashAlgo::Sm3);
        assert_eq!(state.input_format, InputFormat::Hex);
        assert!(state.input_text.is_empty());
        assert!(state.output_hex.is_empty());
        assert!(state.error.is_none());
    }
}

use super::registry::AlgorithmCategory;
use super::tool_trait::CryptoTool;

impl CryptoTool for HashToolState {
    fn name(&self) -> &str { "哈希算法" }
    fn category(&self) -> AlgorithmCategory { AlgorithmCategory::Hash }
    fn execute(&mut self) { self.compute(); }
    fn reset(&mut self) { HashToolState::reset(self); }
    fn has_output(&self) -> bool { !self.output_hex.is_empty() }
    fn output_display(&self) -> String { self.output_hex.clone() }
    fn error_display(&self) -> Option<&str> { self.error.as_deref() }
}
