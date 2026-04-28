#![allow(dead_code)]
use aes::cipher::{BlockDecrypt, BlockDecryptMut, BlockEncrypt, BlockEncryptMut, KeyInit, KeyIvInit};

/// Supported symmetric algorithms
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymmetricAlgo {
    Aes128Ecb,
    Aes256Cbc,
    Sm4Ecb,
    Sm4Cbc,
}

impl std::fmt::Display for SymmetricAlgo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymmetricAlgo::Aes128Ecb => write!(f, "AES-128-ECB"),
            SymmetricAlgo::Aes256Cbc => write!(f, "AES-256-CBC"),
            SymmetricAlgo::Sm4Ecb => write!(f, "SM4-ECB"),
            SymmetricAlgo::Sm4Cbc => write!(f, "SM4-CBC"),
        }
    }
}

impl SymmetricAlgo {
    pub fn key_size(&self) -> usize {
        match self { Self::Aes128Ecb => 16, Self::Aes256Cbc => 32, Self::Sm4Ecb => 16, Self::Sm4Cbc => 16 }
    }
    pub fn needs_iv(&self) -> bool { matches!(self, Self::Aes256Cbc | Self::Sm4Cbc) }
    pub fn all() -> &'static [Self] { &[Self::Aes128Ecb, Self::Aes256Cbc, Self::Sm4Ecb, Self::Sm4Cbc] }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CryptoMode { Encrypt, Decrypt }

impl std::fmt::Display for CryptoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { CryptoMode::Encrypt => write!(f, "加密"), CryptoMode::Decrypt => write!(f, "解密") }
    }
}

pub struct SymmetricToolState {
    pub selected_algo: SymmetricAlgo,
    pub input_hex: String,
    pub key_hex: String,
    pub iv_hex: String,
    pub output_hex: String,
    pub mode: CryptoMode,
    pub error: Option<String>,
}

impl Default for SymmetricToolState {
    fn default() -> Self {
        Self {
            selected_algo: SymmetricAlgo::Aes128Ecb,
            input_hex: String::new(), key_hex: String::new(), iv_hex: String::new(),
            output_hex: String::new(), mode: CryptoMode::Encrypt, error: None,
        }
    }
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = hex.chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    if cleaned.len() % 2 != 0 { return Err("十六进制字符串长度必须为偶数".to_string()); }
    (0..cleaned.len()).step_by(2).map(|i| u8::from_str_radix(&cleaned[i..i+2], 16).map_err(|_| format!("无效的十六进制字符"))).collect()
}
fn hex_encode(data: &[u8]) -> String { data.iter().map(|b| format!("{:02x}", b)).collect() }
fn pkcs7_pad(data: &[u8], bs: usize) -> Vec<u8> {
    let pad = bs - (data.len() % bs);
    let mut out = data.to_vec();
    out.extend(std::iter::repeat(pad as u8).take(pad));
    out
}
fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() { return Err("数据为空".to_string()); }
    let p = data[data.len()-1] as usize;
    if p == 0 || p > data.len() || data[data.len()-p..].iter().any(|&b| b as usize != p) {
        return Err("无效的 PKCS#7 填充".to_string());
    }
    Ok(data[..data.len()-p].to_vec())
}
fn xor_blocks(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut r = [0u8; 16];
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() { r[i] = x ^ y; }
    r
}

// ===== AES =====
fn aes128_ecb_encrypt(pt: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let padded = pkcs7_pad(pt, 16);
    let cipher = aes::Aes128Enc::new_from_slice(key).map_err(|e| format!("创建密钥失败: {}", e))?;
    let mut out = Vec::with_capacity(padded.len());
    for chunk in padded.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let mut gb = aes::cipher::generic_array::GenericArray::<u8, aes::cipher::consts::U16>::clone_from_slice(&block);
        cipher.encrypt_block(&mut gb);
        out.extend_from_slice(&gb);
    }
    Ok(out)
}
fn aes128_ecb_decrypt(ct: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = aes::Aes128Dec::new_from_slice(key).map_err(|e| format!("创建密钥失败: {}", e))?;
    let mut out = Vec::with_capacity(ct.len());
    for chunk in ct.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let mut gb = aes::cipher::generic_array::GenericArray::<u8, aes::cipher::consts::U16>::clone_from_slice(&block);
        cipher.decrypt_block(&mut gb);
        out.extend_from_slice(&gb);
    }
    pkcs7_unpad(&out)
}
fn aes256_cbc_encrypt(pt: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let padded = pkcs7_pad(pt, 16);
    let mut cipher = cbc::Encryptor::<aes::Aes256>::new_from_slices(key, iv).map_err(|e| format!("创建密钥失败: {}", e))?;
    let mut blocks: Vec<aes::cipher::generic_array::GenericArray<u8, aes::cipher::consts::U16>> =
        padded.chunks(16).map(|c| {
            let mut b = [0u8; 16];
            b.copy_from_slice(c);
            aes::cipher::generic_array::GenericArray::clone_from_slice(&b)
        }).collect();
    cipher.encrypt_blocks_mut(&mut blocks);
    let out: Vec<u8> = blocks.iter().flat_map(|b| b.iter().copied()).collect();
    Ok(out)
}
fn aes256_cbc_decrypt(ct: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut cipher = cbc::Decryptor::<aes::Aes256Dec>::new_from_slices(key, iv).map_err(|e| format!("创建密钥失败: {}", e))?;
    let mut blocks: Vec<aes::cipher::generic_array::GenericArray<u8, aes::cipher::consts::U16>> =
        ct.chunks(16).map(|c| {
            let mut b = [0u8; 16];
            b.copy_from_slice(c);
            aes::cipher::generic_array::GenericArray::clone_from_slice(&b)
        }).collect();
    cipher.decrypt_blocks_mut(&mut blocks);
    let out: Vec<u8> = blocks.iter().flat_map(|b| b.iter().copied()).collect();
    pkcs7_unpad(&out)
}

// ===== SM4 (standalone implementation) =====
const SBOX: [u8; 256] = [
    0xd6,0x90,0xe9,0xfe,0xcc,0xe1,0x3d,0xb7,0x16,0xb6,0x14,0xc2,0x28,0xfb,0x2c,0x05,
    0x2b,0x67,0x9a,0x76,0x2a,0xbe,0x04,0xc3,0xaa,0x44,0x13,0x26,0x49,0x86,0x06,0x99,
    0x9c,0x42,0x50,0xf4,0x91,0xef,0x98,0x7a,0x33,0x54,0x0b,0x43,0xed,0xcf,0xac,0x62,
    0xe4,0xb3,0x1c,0xa9,0xc9,0x08,0xe8,0x95,0x80,0xdf,0x94,0xfa,0x75,0x8f,0x3f,0xa6,
    0x47,0x07,0xa7,0xfc,0xf3,0x73,0x17,0xba,0x83,0x59,0x3c,0x19,0xe6,0x85,0x4f,0xa8,
    0x68,0x6b,0x81,0xb2,0x71,0x64,0xda,0x8b,0xf8,0xeb,0x0f,0x4b,0x70,0x56,0x9d,0x35,
    0x1e,0x24,0x0e,0x5e,0x63,0x58,0xd1,0xa2,0x25,0x22,0x7c,0x3b,0x01,0x21,0x78,0x87,
    0xd4,0x00,0x46,0x57,0x9f,0xd3,0x27,0x52,0x4c,0x36,0x02,0xe7,0xa0,0xc4,0xc8,0x9e,
    0xea,0xbf,0x8a,0xd2,0x40,0xc7,0x38,0xb5,0xa3,0xf7,0xf2,0xce,0xf9,0x61,0x15,0xa1,
    0xe0,0xae,0x5d,0xa4,0x9b,0x34,0x1a,0x55,0xad,0x93,0x32,0x30,0xf5,0x8c,0xb1,0xe3,
    0x1d,0xf6,0xe2,0x2e,0x82,0x66,0xca,0x60,0xc0,0x29,0x23,0xab,0x0d,0x53,0x4e,0x6f,
    0xd5,0xdb,0x37,0x45,0xde,0xfd,0x8e,0x2f,0x03,0xff,0x6a,0x72,0x6d,0x6c,0x5b,0x51,
    0x8d,0x1b,0xaf,0x92,0xbb,0xdd,0xbc,0x7f,0x11,0xd9,0x5c,0x41,0x1f,0x10,0x5a,0xd8,
    0x0a,0xc1,0x31,0x88,0xa5,0xcd,0x7b,0xbd,0x2d,0x74,0xd0,0x12,0xb8,0xe5,0xb4,0xb0,
    0x89,0x69,0x97,0x4a,0x0c,0x96,0x77,0x7e,0x65,0xb9,0xf1,0x09,0xc5,0x6e,0xc6,0x84,
    0x18,0xf0,0x7d,0xec,0x3a,0xdc,0x4d,0x20,0x79,0xee,0x5f,0x3e,0xd7,0xcb,0x39,0x48,
];

fn sm4_t_prime(v: u32) -> u32 {
    let b = v.to_be_bytes();
    let t = u32::from_be_bytes([SBOX[b[0] as usize], SBOX[b[1] as usize], SBOX[b[2] as usize], SBOX[b[3] as usize]]);
    t ^ t.rotate_left(13) ^ t.rotate_left(23)
}
fn sm4_round(v: u32) -> u32 {
    let b = v.to_be_bytes();
    let t = u32::from_be_bytes([SBOX[b[0] as usize], SBOX[b[1] as usize], SBOX[b[2] as usize], SBOX[b[3] as usize]]);
    t ^ t.rotate_left(2) ^ t.rotate_left(10) ^ t.rotate_left(18) ^ t.rotate_left(24)
}
fn sm4_key_schedule(key: &[u8]) -> [u32; 32] {
    let mk = [u32::from_be_bytes(key[0..4].try_into().unwrap()), u32::from_be_bytes(key[4..8].try_into().unwrap()),
              u32::from_be_bytes(key[8..12].try_into().unwrap()), u32::from_be_bytes(key[12..16].try_into().unwrap())];
    let fk = [0xA3B1BAC6, 0x56AA3350, 0x677D9197, 0xB27022DC];
    let ck: [u32; 32] = [
        0x00070E15,0x1C232A31,0x383F464D,0x545B6269,0x70777E85,0x8C939AA1,0xA8AFB6BD,0xC4CBD2D9,
        0xE0E7EEF5,0xFC030A11,0x181F262D,0x343B4249,0x50575E65,0x6C737A81,0x888F969D,0xA4ABB2B9,
        0xC0C7CED5,0xDCE3EAF1,0xF8FF060D,0x141B2229,0x30373E45,0x4C535A61,0x686F767D,0x848B9299,
        0xA0A7AEB5,0xBCC3CAD1,0xD8DFE6ED,0xF4FB0209,0x10171E25,0x2C333A41,0x484F565D,0x646B7279];
    let mut k = [mk[0]^fk[0], mk[1]^fk[1], mk[2]^fk[2], mk[3]^fk[3]];
    let mut rk = [0u32; 32];
    for i in 0..32 { k[i%4] ^= sm4_t_prime(k[(i+1)%4] ^ k[(i+2)%4] ^ k[(i+3)%4] ^ ck[i]); rk[i] = k[i%4]; }
    rk
}
fn sm4_block(block: &mut [u8; 16], rk: &[u32; 32]) {
    let mut x = [u32::from_be_bytes(block[0..4].try_into().unwrap()), u32::from_be_bytes(block[4..8].try_into().unwrap()),
                 u32::from_be_bytes(block[8..12].try_into().unwrap()), u32::from_be_bytes(block[12..16].try_into().unwrap())];
    for i in 0..32 { x[i%4] ^= sm4_round(x[(i+1)%4] ^ x[(i+2)%4] ^ x[(i+3)%4] ^ rk[i]); }
    block[0..4].copy_from_slice(&x[3].to_be_bytes()); block[4..8].copy_from_slice(&x[2].to_be_bytes());
    block[8..12].copy_from_slice(&x[1].to_be_bytes()); block[12..16].copy_from_slice(&x[0].to_be_bytes());
}
fn sm4_ecb_encrypt(pt: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let padded = pkcs7_pad(pt, 16);
    let rk = sm4_key_schedule(key);
    let mut out = Vec::with_capacity(padded.len());
    for chunk in padded.chunks(16) { let mut b = [0u8; 16]; b.copy_from_slice(chunk); sm4_block(&mut b, &rk); out.extend_from_slice(&b); }
    Ok(out)
}
fn sm4_ecb_decrypt(ct: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let rk = sm4_key_schedule(key);
    let mut drk = rk; drk.reverse();
    let mut out = Vec::with_capacity(ct.len());
    for chunk in ct.chunks(16) { let mut b = [0u8; 16]; b.copy_from_slice(chunk); sm4_block(&mut b, &drk); out.extend_from_slice(&b); }
    pkcs7_unpad(&out)
}
fn sm4_cbc_encrypt(pt: &[u8], key: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>, String> {
    let padded = pkcs7_pad(pt, 16);
    let rk = sm4_key_schedule(key);
    let mut out = Vec::with_capacity(padded.len());
    let mut prev = *iv;
    for chunk in padded.chunks(16) { let mut b = [0u8; 16]; b.copy_from_slice(chunk); let x = xor_blocks(&prev, &b); let mut x = x; sm4_block(&mut x, &rk); prev = x; out.extend_from_slice(&x); }
    Ok(out)
}
fn sm4_cbc_decrypt(ct: &[u8], key: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>, String> {
    let rk = sm4_key_schedule(key);
    let mut drk = rk; drk.reverse();
    let mut out = Vec::with_capacity(ct.len());
    let mut prev = *iv;
    for chunk in ct.chunks(16) { let mut b = [0u8; 16]; b.copy_from_slice(chunk); let cb = b; sm4_block(&mut b, &drk); let p = xor_blocks(&prev, &b); prev = cb; out.extend_from_slice(&p); }
    pkcs7_unpad(&out)
}

impl SymmetricToolState {
    pub fn execute(&mut self) {
        self.error = None; self.output_hex.clear();
        let input = match hex_decode(&self.input_hex) { Ok(b) => b, Err(e) => { self.error = Some(format!("输入数据错误: {}", e)); return; } };
        let key = match hex_decode(&self.key_hex) { Ok(b) => b, Err(e) => { self.error = Some(format!("密钥错误: {}", e)); return; } };
        if key.len() != self.selected_algo.key_size() {
            self.error = Some(format!("密钥长度不正确。{} 需要 {} 字节密钥，当前 {} 字节", self.selected_algo, self.selected_algo.key_size(), key.len()));
            return;
        }
        let result = match self.selected_algo {
            SymmetricAlgo::Aes128Ecb => match self.mode {
                CryptoMode::Encrypt => aes128_ecb_encrypt(&input, &key),
                CryptoMode::Decrypt => aes128_ecb_decrypt(&input, &key),
            },
            SymmetricAlgo::Aes256Cbc => {
                let iv = match hex_decode(&self.iv_hex) { Ok(b) if b.len() == 16 => { let mut a = [0u8; 16]; a.copy_from_slice(&b); a } _ => { self.error = Some("IV 长度必须为 16 字节".to_string()); return; } };
                match self.mode { CryptoMode::Encrypt => aes256_cbc_encrypt(&input, &key, &iv), CryptoMode::Decrypt => aes256_cbc_decrypt(&input, &key, &iv) }
            }
            SymmetricAlgo::Sm4Ecb => match self.mode {
                CryptoMode::Encrypt => sm4_ecb_encrypt(&input, &key),
                CryptoMode::Decrypt => sm4_ecb_decrypt(&input, &key),
            },
            SymmetricAlgo::Sm4Cbc => {
                let iv = match hex_decode(&self.iv_hex) { Ok(b) if b.len() == 16 => { let mut a = [0u8; 16]; a.copy_from_slice(&b); a } _ => { self.error = Some("IV 长度必须为 16 字节".to_string()); return; } };
                match self.mode { CryptoMode::Encrypt => sm4_cbc_encrypt(&input, &key, &iv), CryptoMode::Decrypt => sm4_cbc_decrypt(&input, &key, &iv) }
            }
        };
        match result { Ok(out) => self.output_hex = hex_encode(&out), Err(e) => self.error = Some(e) }
    }
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode { CryptoMode::Encrypt => CryptoMode::Decrypt, _ => CryptoMode::Encrypt };
        self.output_hex.clear(); self.error = None;
    }
    pub fn select_algo(&mut self, algo: SymmetricAlgo) {
        self.selected_algo = algo; self.key_hex.clear(); self.iv_hex.clear(); self.input_hex.clear(); self.output_hex.clear(); self.error = None;
    }
    pub fn reset(&mut self) {
        self.input_hex.clear();
        self.key_hex.clear();
        self.iv_hex.clear();
        self.output_hex.clear();
        self.error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aes128_ecb_roundtrip() {
        let key = [0x00u8; 16];
        let pt = b"Hello, World!!!";
        let ct = aes128_ecb_encrypt(pt, &key).unwrap();
        let dec = aes128_ecb_decrypt(&ct, &key).unwrap();
        assert_eq!(dec, pt);
    }
    #[test]
    fn test_sm4_ecb_roundtrip() {
        let key = [0x00u8; 16];
        let pt = b"Hello, World!!!";
        let ct = sm4_ecb_encrypt(pt, &key).unwrap();
        let dec = sm4_ecb_decrypt(&ct, &key).unwrap();
        assert_eq!(dec, pt);
    }

    #[test]
    fn reset_clears_data_and_preserves_selected_algorithm_and_mode() {
        let mut state = SymmetricToolState {
            selected_algo: SymmetricAlgo::Sm4Cbc,
            input_hex: "001122".to_string(),
            key_hex: "00".repeat(16),
            iv_hex: "11".repeat(16),
            output_hex: "deadbeef".to_string(),
            mode: CryptoMode::Decrypt,
            error: Some("错误".to_string()),
        };

        state.reset();

        assert_eq!(state.selected_algo, SymmetricAlgo::Sm4Cbc);
        assert_eq!(state.mode, CryptoMode::Decrypt);
        assert!(state.input_hex.is_empty());
        assert!(state.key_hex.is_empty());
        assert!(state.iv_hex.is_empty());
        assert!(state.output_hex.is_empty());
        assert!(state.error.is_none());
    }
}
