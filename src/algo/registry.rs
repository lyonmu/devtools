#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt;

use crate::algo::oid_defs;

#[derive(Clone, Debug)]
pub struct AlgorithmInfo {
    pub oid: &'static str,
    pub name: &'static str,
    pub category: AlgorithmCategory,
    pub parameters: Vec<(&'static str, &'static str)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlgorithmCategory {
    Hash,
    Asymmetric,
    Symmetric,
    Signature,
    KEM,
}

impl fmt::Display for AlgorithmCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hash => write!(f, "Hash"),
            Self::Asymmetric => write!(f, "非对称"),
            Self::Symmetric => write!(f, "对称"),
            Self::Signature => write!(f, "签名"),
            Self::KEM => write!(f, "密钥封装"),
        }
    }
}

pub struct AlgorithmRegistry {
    algorithms: Vec<AlgorithmInfo>,
    name_index: HashMap<&'static str, usize>,
    oid_index: HashMap<&'static str, usize>,
}

impl AlgorithmRegistry {
    pub fn new() -> Self {
        let algorithms = build_algorithm_table();
        let mut name_index = HashMap::new();
        let mut oid_index = HashMap::new();
        for (i, algo) in algorithms.iter().enumerate() {
            name_index.insert(algo.name, i);
            oid_index.insert(algo.oid, i);
        }
        Self {
            algorithms,
            name_index,
            oid_index,
        }
    }

    pub fn lookup_by_oid(&self, oid: &str) -> Option<&AlgorithmInfo> {
        self.oid_index.get(oid).map(|&i| &self.algorithms[i])
    }

    pub fn lookup_by_name(&self, name: &str) -> Option<&AlgorithmInfo> {
        self.name_index.get(name).map(|&i| &self.algorithms[i])
    }

    pub fn all(&self) -> &[AlgorithmInfo] {
        &self.algorithms
    }
}

/// Standard OID constants (SHA family, PKCS#1, etc.)
pub mod std_oid {
    pub const SHA1: &str = "1.3.14.3.2.26";
    pub const SHA256: &str = "2.16.840.1.101.3.4.2.1";
    pub const SHA384: &str = "2.16.840.1.101.3.4.2.2";
    pub const SHA512: &str = "2.16.840.1.101.3.4.2.3";
}

fn build_algorithm_table() -> Vec<AlgorithmInfo> {
    vec![
        // === Hash ===
        AlgorithmInfo {
            oid: std_oid::SHA256,
            name: "sha256",
            category: AlgorithmCategory::Hash,
            parameters: vec![
                ("Digest Size", "256 bits"),
                ("Block Size", "512 bits"),
                ("Standard", "FIPS 180-4"),
            ],
        },
        AlgorithmInfo {
            oid: std_oid::SHA384,
            name: "sha384",
            category: AlgorithmCategory::Hash,
            parameters: vec![
                ("Digest Size", "384 bits"),
                ("Block Size", "1024 bits"),
                ("Standard", "FIPS 180-4"),
            ],
        },
        AlgorithmInfo {
            oid: std_oid::SHA512,
            name: "sha512",
            category: AlgorithmCategory::Hash,
            parameters: vec![
                ("Digest Size", "512 bits"),
                ("Block Size", "1024 bits"),
                ("Standard", "FIPS 180-4"),
            ],
        },
        AlgorithmInfo {
            oid: std_oid::SHA1,
            name: "sha1",
            category: AlgorithmCategory::Hash,
            parameters: vec![
                ("Digest Size", "160 bits"),
                ("Block Size", "512 bits"),
                ("Standard", "FIPS 180-4 (deprecated)"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::gmt::SM3,
            name: "SM3",
            category: AlgorithmCategory::Hash,
            parameters: vec![
                ("Digest Size", "256 bits"),
                ("Block Size", "512 bits"),
                ("Standard", "GM/T 0004-2012"),
            ],
        },

        // === Asymmetric ===
        AlgorithmInfo {
            oid: "1.2.840.113549.1.1.1",
            name: "rsaEncryption",
            category: AlgorithmCategory::Asymmetric,
            parameters: vec![
                ("Key Size", "1024-4096 bits"),
                ("Padding Scheme", "PKCS#1 v1.5, OAEP"),
                ("Standard", "PKCS#1"),
            ],
        },
        AlgorithmInfo {
            oid: "1.2.840.10045.2.1",
            name: "ecPublicKey",
            category: AlgorithmCategory::Asymmetric,
            parameters: vec![
                ("Key Size", "256-521 bits"),
                ("Curve", "P-256, P-384, P-521"),
                ("Standard", "X9.62"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::gmt::SM2,
            name: "SM2",
            category: AlgorithmCategory::Asymmetric,
            parameters: vec![
                ("Key Size", "256 bits"),
                ("Curve", "SM2 curve"),
                ("Standard", "GM/T 0003-2012"),
            ],
        },

        // === Symmetric ===
        AlgorithmInfo {
            oid: "2.16.840.1.101.3.4.1.1",
            name: "aes128-ECB",
            category: AlgorithmCategory::Symmetric,
            parameters: vec![
                ("Key Size", "128 bits"),
                ("Mode", "ECB"),
                ("Standard", "FIPS 197"),
            ],
        },
        AlgorithmInfo {
            oid: "2.16.840.1.101.3.4.1.42",
            name: "aes256-CBC",
            category: AlgorithmCategory::Symmetric,
            parameters: vec![
                ("Key Size", "256 bits"),
                ("Mode", "CBC"),
                ("Standard", "FIPS 197"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::gmt::SM4,
            name: "SM4",
            category: AlgorithmCategory::Symmetric,
            parameters: vec![
                ("Key Size", "128 bits"),
                ("Block Size", "128 bits"),
                ("Mode", "ECB/CBC/CFB/OFB"),
                ("Standard", "GM/T 0002-2012"),
            ],
        },

        // === Signature ===
        AlgorithmInfo {
            oid: "1.2.840.113549.1.1.11",
            name: "sha256WithRSAEncryption",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Key Size", "2048-4096 bits"),
                ("Padding Scheme", "PKCS#1 v1.5"),
                ("Standard", "PKCS#1"),
            ],
        },
        AlgorithmInfo {
            oid: "1.2.840.10045.4.3.2",
            name: "ecdsa-with-SHA256",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Key Size", "256 bits"),
                ("Curve", "P-256"),
                ("Standard", "X9.62"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::ML_DSA_44,
            name: "ML-DSA-44",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "2 (Category 2)"),
                ("Public Key Size", "1312 bytes"),
                ("Signature Size", "2420 bytes"),
                ("Standard", "FIPS 204"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::ML_DSA_65,
            name: "ML-DSA-65",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "3 (Category 3)"),
                ("Public Key Size", "1952 bytes"),
                ("Signature Size", "3309 bytes"),
                ("Standard", "FIPS 204"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::ML_DSA_87,
            name: "ML-DSA-87",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "5 (Category 5)"),
                ("Public Key Size", "2592 bytes"),
                ("Signature Size", "4627 bytes"),
                ("Standard", "FIPS 204"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::SLH_DSA_SHA2_128S,
            name: "SLH-DSA-SHA2-128s",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "1"),
                ("Public Key Size", "32 bytes"),
                ("Signature Size", "7856 bytes"),
                ("Standard", "FIPS 205"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::SLH_DSA_SHA2_256S,
            name: "SLH-DSA-SHA2-256s",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "3"),
                ("Public Key Size", "32 bytes"),
                ("Signature Size", "16976 bytes"),
                ("Standard", "FIPS 205"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::FN_DSA_512,
            name: "FN-DSA-512 (Provisional)",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "1"),
                ("Public Key Size", "897 bytes"),
                ("Signature Size", "666 bytes"),
                ("Standard", "FIPS 206 (draft)"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::FN_DSA_1024,
            name: "FN-DSA-1024 (Provisional)",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "5"),
                ("Public Key Size", "1793 bytes"),
                ("Signature Size", "1280 bytes"),
                ("Standard", "FIPS 206 (draft)"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::FN_DSA_512_FIPS,
            name: "FN-DSA-512 (FIPS 206 predicted)",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "1"),
                ("Public Key Size", "897 bytes"),
                ("Signature Size", "666 bytes"),
                ("Standard", "FIPS 206 (pending)"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::FN_DSA_1024_FIPS,
            name: "FN-DSA-1024 (FIPS 206 predicted)",
            category: AlgorithmCategory::Signature,
            parameters: vec![
                ("Security Level", "5"),
                ("Public Key Size", "1793 bytes"),
                ("Signature Size", "1280 bytes"),
                ("Standard", "FIPS 206 (pending)"),
            ],
        },

        // === KEM ===
        AlgorithmInfo {
            oid: oid_defs::pq::ML_KEM_512,
            name: "ML-KEM-512",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "1 (Category 1)"),
                ("Public Key Size", "800 bytes"),
                ("Ciphertext Size", "768 bytes"),
                ("Shared Secret", "32 bytes"),
                ("Standard", "FIPS 203"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::ML_KEM_768,
            name: "ML-KEM-768",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "3 (Category 3)"),
                ("Public Key Size", "1184 bytes"),
                ("Ciphertext Size", "1088 bytes"),
                ("Shared Secret", "32 bytes"),
                ("Standard", "FIPS 203"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::ML_KEM_1024,
            name: "ML-KEM-1024",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "5 (Category 5)"),
                ("Public Key Size", "1568 bytes"),
                ("Ciphertext Size", "1568 bytes"),
                ("Shared Secret", "32 bytes"),
                ("Standard", "FIPS 203"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::HQC_128,
            name: "HQC-128 (PROVISIONAL)",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "1"),
                ("Public Key Size", "2229 bytes"),
                ("Ciphertext Size", "2322 bytes"),
                ("Standard", "NIST Round 4"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::HQC_192,
            name: "HQC-192 (PROVISIONAL)",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "3"),
                ("Public Key Size", "4669 bytes"),
                ("Ciphertext Size", "4762 bytes"),
                ("Standard", "NIST Round 4"),
            ],
        },
        AlgorithmInfo {
            oid: oid_defs::pq::HQC_256,
            name: "HQC-256 (PROVISIONAL)",
            category: AlgorithmCategory::KEM,
            parameters: vec![
                ("Security Level", "5"),
                ("Public Key Size", "7429 bytes"),
                ("Ciphertext Size", "7522 bytes"),
                ("Standard", "NIST Round 4"),
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_new_creates_with_algorithms() {
        let registry = AlgorithmRegistry::new();
        assert!(!registry.all().is_empty());
        assert!(registry.all().len() > 20, "Expected >20 algorithms, got {}", registry.all().len());
    }

    #[test]
    fn lookup_by_oid_finds_known_algorithms() {
        let registry = AlgorithmRegistry::new();

        // SHA-256
        let sha256 = registry.lookup_by_oid(std_oid::SHA256);
        assert!(sha256.is_some());
        assert_eq!(sha256.unwrap().name, "sha256");
        assert_eq!(sha256.unwrap().category, AlgorithmCategory::Hash);

        // RSA
        let rsa = registry.lookup_by_oid("1.2.840.113549.1.1.1");
        assert!(rsa.is_some());
        assert_eq!(rsa.unwrap().name, "rsaEncryption");
        assert_eq!(rsa.unwrap().category, AlgorithmCategory::Asymmetric);

        // ML-KEM-512
        let ml_kem = registry.lookup_by_oid(oid_defs::pq::ML_KEM_512);
        assert!(ml_kem.is_some());
        assert_eq!(ml_kem.unwrap().name, "ML-KEM-512");
        assert_eq!(ml_kem.unwrap().category, AlgorithmCategory::KEM);
    }

    #[test]
    fn lookup_by_name_finds_known_algorithms() {
        let registry = AlgorithmRegistry::new();

        let sha256 = registry.lookup_by_name("sha256");
        assert!(sha256.is_some());
        assert_eq!(sha256.unwrap().oid, std_oid::SHA256);

        let sm4 = registry.lookup_by_name("SM4");
        assert!(sm4.is_some());
        assert_eq!(sm4.unwrap().oid, oid_defs::gmt::SM4);
    }

    #[test]
    fn lookup_by_oid_returns_none_for_unknown() {
        let registry = AlgorithmRegistry::new();
        assert!(registry.lookup_by_oid("9.9.9.9.9").is_none());
    }

    #[test]
    fn lookup_by_name_returns_none_for_unknown() {
        let registry = AlgorithmRegistry::new();
        assert!(registry.lookup_by_name("unknown-algorithm").is_none());
    }

    #[test]
    fn all_returns_all_categories() {
        let registry = AlgorithmRegistry::new();
        let all = registry.all();

        let has_hash = all.iter().any(|a| a.category == AlgorithmCategory::Hash);
        let has_asymmetric = all.iter().any(|a| a.category == AlgorithmCategory::Asymmetric);
        let has_symmetric = all.iter().any(|a| a.category == AlgorithmCategory::Symmetric);
        let has_signature = all.iter().any(|a| a.category == AlgorithmCategory::Signature);
        let has_kem = all.iter().any(|a| a.category == AlgorithmCategory::KEM);

        assert!(has_hash, "No Hash algorithms");
        assert!(has_asymmetric, "No Asymmetric algorithms");
        assert!(has_symmetric, "No Symmetric algorithms");
        assert!(has_signature, "No Signature algorithms");
        assert!(has_kem, "No KEM algorithms");
    }

    #[test]
    fn algorithm_category_display() {
        assert_eq!(AlgorithmCategory::Hash.to_string(), "Hash");
        assert_eq!(AlgorithmCategory::Asymmetric.to_string(), "非对称");
        assert_eq!(AlgorithmCategory::Symmetric.to_string(), "对称");
        assert_eq!(AlgorithmCategory::Signature.to_string(), "签名");
        assert_eq!(AlgorithmCategory::KEM.to_string(), "密钥封装");
    }

    #[test]
    fn algorithm_info_has_required_fields() {
        let registry = AlgorithmRegistry::new();
        for algo in registry.all() {
            assert!(!algo.oid.is_empty(), "Algorithm {} has empty OID", algo.name);
            assert!(!algo.name.is_empty(), "Algorithm {} has empty name", algo.oid);
            assert!(!algo.parameters.is_empty(), "Algorithm {} has no parameters", algo.name);
        }
    }
}
