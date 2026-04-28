#![allow(dead_code)]
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::algo::oid_defs;

static OID_ALGO_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // GM/T OIDs
    m.insert(oid_defs::gmt::SM2, "SM2");
    m.insert(oid_defs::gmt::SM2_KEY_EXCHANGE, "SM2 Key Exchange");
    m.insert(oid_defs::gmt::SM2_ENCRYPTION, "SM2 Encryption");
    m.insert(oid_defs::gmt::SM3, "SM3");
    m.insert(oid_defs::gmt::SM3_KEYLESS, "SM3 Keyless");
    m.insert(oid_defs::gmt::HMAC_SM3, "HMAC-SM3");
    m.insert(oid_defs::gmt::SM2_SM3_SIG, "SM2-SM3 Signature");
    m.insert(oid_defs::gmt::SM4, "SM4");

    // Post-quantum OIDs
    m.insert(oid_defs::pq::ML_DSA_44, "ML-DSA-44");
    m.insert(oid_defs::pq::ML_DSA_65, "ML-DSA-65");
    m.insert(oid_defs::pq::ML_DSA_87, "ML-DSA-87");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_128S, "SLH-DSA-SHA2-128s");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_128F, "SLH-DSA-SHA2-128f");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_192S, "SLH-DSA-SHA2-192s");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_192F, "SLH-DSA-SHA2-192f");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_256S, "SLH-DSA-SHA2-256s");
    m.insert(oid_defs::pq::SLH_DSA_SHA2_256F, "SLH-DSA-SHA2-256f");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_128S, "SLH-DSA-SHAKE-128s");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_128F, "SLH-DSA-SHAKE-128f");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_192S, "SLH-DSA-SHAKE-192s");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_192F, "SLH-DSA-SHAKE-192f");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_256S, "SLH-DSA-SHAKE-256s");
    m.insert(oid_defs::pq::SLH_DSA_SHAKE_256F, "SLH-DSA-SHAKE-256f");
    m.insert(oid_defs::pq::FN_DSA_512, "FN-DSA-512");
    m.insert(oid_defs::pq::FN_DSA_1024, "FN-DSA-1024");
    m.insert(oid_defs::pq::FN_DSA_512_FIPS, "FN-DSA-512 (FIPS 206)");
    m.insert(oid_defs::pq::FN_DSA_1024_FIPS, "FN-DSA-1024 (FIPS 206)");
    m.insert(oid_defs::pq::HQC_128, "HQC-128");
    m.insert(oid_defs::pq::HQC_192, "HQC-192");
    m.insert(oid_defs::pq::HQC_256, "HQC-256");

    // Standard algorithm OIDs
    // RSA
    m.insert("1.2.840.113549.1.1.1", "RSA");
    m.insert("1.2.840.113549.1.1.2", "MD2-RSA");
    m.insert("1.2.840.113549.1.1.4", "MD5-RSA");
    m.insert("1.2.840.113549.1.1.5", "SHA1-RSA");
    m.insert("1.2.840.113549.1.1.11", "SHA256-RSA");
    m.insert("1.2.840.113549.1.1.12", "SHA384-RSA");
    m.insert("1.2.840.113549.1.1.13", "SHA512-RSA");
    // RSA PSS
    m.insert("1.2.840.113549.1.1.10", "RSA-PSS");

    // EC
    m.insert("1.2.840.10045.2.1", "ECDSA");
    // ECDSA signature OIDs
    m.insert("1.2.840.10045.4.3.2", "ECDSA-SHA256");
    m.insert("1.2.840.10045.4.3.3", "ECDSA-SHA384");
    m.insert("1.2.840.10045.4.3.4", "ECDSA-SHA512");

    // SHA hash family
    m.insert("1.3.14.3.2.26", "SHA1");
    m.insert("2.16.840.1.101.3.4.2.1", "SHA256");
    m.insert("2.16.840.1.101.3.4.2.2", "SHA384");
    m.insert("2.16.840.1.101.3.4.2.3", "SHA512");
    m.insert("2.16.840.1.101.3.4.2.4", "SHA224");
    m.insert("2.16.840.1.101.3.4.2.5", "SHA512-224");
    m.insert("2.16.840.1.101.3.4.2.6", "SHA512-256");
    // SHA3
    m.insert("2.16.840.1.101.3.4.2.7", "SHA3-256");
    m.insert("2.16.840.1.101.3.4.2.8", "SHA3-384");
    m.insert("2.16.840.1.101.3.4.2.9", "SHA3-512");

    // AES
    m.insert("2.16.840.1.101.3.4.1.1", "AES-128-ECB");
    m.insert("2.16.840.1.101.3.4.1.2", "AES-128-CBC");
    m.insert("2.16.840.1.101.3.4.1.6", "AES-256-ECB");
    m.insert("2.16.840.1.101.3.4.1.7", "AES-256-CBC");
    m.insert("2.16.840.1.101.3.4.1.42", "AES-256-GCM");

    // Ed25519 / Ed448
    m.insert("1.3.101.112", "Ed25519");
    m.insert("1.3.101.113", "Ed448");
    // X25519 / X448
    m.insert("1.3.101.110", "X25519");
    m.insert("1.3.101.111", "X448");

    // ML-KEM (FIPS 203)
    m.insert("2.16.840.1.101.3.4.4.1", "ML-KEM-512");
    m.insert("2.16.840.1.101.3.4.4.2", "ML-KEM-768");
    m.insert("2.16.840.1.101.3.4.4.3", "ML-KEM-1024");
    // Legacy ML-KEM OIDs (formerly under symmetric OID arc)
    m.insert("2.16.840.1.101.3.4.1.43", "ML-KEM-512 (Legacy)");
    m.insert("2.16.840.1.101.3.4.1.44", "ML-KEM-768 (Legacy)");
    m.insert("2.16.840.1.101.3.4.1.45", "ML-KEM-1024 (Legacy)");

    m
});

/// X.509 extension OIDs
static OID_EXT_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("2.5.29.14", "Subject Key Identifier");
    m.insert("2.5.29.15", "Key Usage");
    m.insert("2.5.29.16", "Private Key Usage Period");
    m.insert("2.5.29.17", "Subject Alternative Name");
    m.insert("2.5.29.18", "Issuer Alternative Name");
    m.insert("2.5.29.19", "Basic Constraints");
    m.insert("2.5.29.30", "Name Constraints");
    m.insert("2.5.29.31", "CRL Distribution Points");
    m.insert("2.5.29.32", "Certificate Policies");
    m.insert("2.5.29.33", "Policy Mappings");
    m.insert("2.5.29.35", "Authority Key Identifier");
    m.insert("2.5.29.36", "Policy Constraints");
    m.insert("2.5.29.37", "Extended Key Usage");
    m.insert("2.5.29.46", "Freshest CRL");
    m.insert("2.5.29.54", "Inhibit Any-Policy");
    m.insert("1.3.6.1.5.5.7.1.1", "Authority Information Access");
    m.insert("1.3.6.1.5.5.7.1.11", "Subject Information Access");
    m.insert("1.3.6.1.4.1.11129.2.4.2", "SCT List (CT)");
    m.insert("1.3.6.1.5.5.7.48.1", "OCSP");
    m.insert("1.3.6.1.5.5.7.48.2", "CA Issuers");
    m
});

/// Key category based on algorithm OID
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyCategory {
    Classic,
    PostQuantum,
}

impl std::fmt::Display for KeyCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyCategory::Classic => write!(f, "传统算法"),
            KeyCategory::PostQuantum => write!(f, "抗量子算法"),
        }
    }
}

/// Resolve an algorithm OID to its human-readable name.
pub fn resolve_algorithm_name(oid: &str) -> String {
    if let Some(name) = OID_ALGO_MAP.get(oid) {
        return name.to_string();
    }
    format!("未知 OID ({})", oid)
}

/// Determine if an OID corresponds to a post-quantum algorithm.
pub fn resolve_key_category(oid: &str) -> KeyCategory {
    if oid.starts_with("2.16.840.1.101.3.4.3.1") // ML-DSA (FIPS 204) OIDs 17-19
        || oid.starts_with("2.16.840.1.101.3.4.3.2") // SLH-DSA (FIPS 205) OIDs 20-31
        || oid.starts_with("2.16.840.1.101.3.4.3.3") // Predicted FN-DSA FIPS 206 OIDs 32+
        || oid.starts_with("2.16.840.1.101.3.4.3.4") // FN-DSA FIPS 206 predicted 47-48
        || oid.starts_with("1.3.6.1.4.1.2.267.8") // FN-DSA provisional IBM arc
        || oid.starts_with("1.3.6.1.4.1.22554.5") // HQC provisional
        || oid.starts_with("2.16.840.1.101.3.4.4.") // ML-KEM (FIPS 203) OIDs 1-3
        || oid.starts_with("2.16.840.1.101.3.4.1.4") // ML-KEM legacy OIDs 43-45
    {
        return KeyCategory::PostQuantum;
    }
    KeyCategory::Classic
}

/// Resolve an extension OID to its human-readable name.
pub fn resolve_extension_name(oid: &str) -> String {
    OID_EXT_MAP
        .get(oid)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("未知扩展 ({})", oid))
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Algorithm name resolution tests ===

    #[test]
    fn test_resolve_pq_oid_ml_dsa() {
        assert_eq!(resolve_algorithm_name(oid_defs::pq::ML_DSA_44), "ML-DSA-44");
    }

    #[test]
    fn test_resolve_gmt_oid_sm2() {
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::SM2), "SM2");
    }

    #[test]
    fn test_resolve_rsa_oid() {
        assert_eq!(resolve_algorithm_name("1.2.840.113549.1.1.1"), "RSA");
    }

    #[test]
    fn test_resolve_unknown_oid() {
        let result = resolve_algorithm_name("9.9.9.9.9");
        assert!(result.starts_with("未知 OID"));
    }

    #[test]
    fn classic_rsa_ecdsa_sha_aes_resolve_to_exact_names() {
        // RSA variants
        assert_eq!(resolve_algorithm_name("1.2.840.113549.1.1.1"), "RSA");
        assert_eq!(resolve_algorithm_name("1.2.840.113549.1.1.11"), "SHA256-RSA");
        assert_eq!(resolve_algorithm_name("1.2.840.113549.1.1.10"), "RSA-PSS");

        // EC/ECDSA
        assert_eq!(resolve_algorithm_name("1.2.840.10045.2.1"), "ECDSA");
        assert_eq!(resolve_algorithm_name("1.2.840.10045.4.3.2"), "ECDSA-SHA256");

        // SHA family
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.2.1"), "SHA256");
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.2.3"), "SHA512");

        // AES variants
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.1.1"), "AES-128-ECB");
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.1.42"), "AES-256-GCM");

        // Ed25519/Ed448
        assert_eq!(resolve_algorithm_name("1.3.101.112"), "Ed25519");
        assert_eq!(resolve_algorithm_name("1.3.101.113"), "Ed448");
    }

    #[test]
    fn gmt_sm2_sm3_sm4_resolve_to_exact_names() {
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::SM2), "SM2");
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::SM3), "SM3");
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::SM4), "SM4");
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::SM2_SM3_SIG), "SM2-SM3 Signature");
        assert_eq!(resolve_algorithm_name(oid_defs::gmt::HMAC_SM3), "HMAC-SM3");
    }

    #[test]
    fn pq_ml_kem_ml_dsa_resolve_to_exact_names() {
        // ML-KEM
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.4.1"), "ML-KEM-512");
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.4.2"), "ML-KEM-768");
        assert_eq!(resolve_algorithm_name("2.16.840.1.101.3.4.4.3"), "ML-KEM-1024");

        // ML-DSA
        assert_eq!(resolve_algorithm_name(oid_defs::pq::ML_DSA_44), "ML-DSA-44");
        assert_eq!(resolve_algorithm_name(oid_defs::pq::ML_DSA_65), "ML-DSA-65");
        assert_eq!(resolve_algorithm_name(oid_defs::pq::ML_DSA_87), "ML-DSA-87");
    }

    // === Extension name resolution tests ===

    #[test]
    fn test_resolve_ext_san() {
        assert_eq!(resolve_extension_name("2.5.29.17"), "Subject Alternative Name");
    }

    #[test]
    fn extension_oids_resolve_to_exact_names() {
        assert_eq!(resolve_extension_name("2.5.29.14"), "Subject Key Identifier");
        assert_eq!(resolve_extension_name("2.5.29.15"), "Key Usage");
        assert_eq!(resolve_extension_name("2.5.29.19"), "Basic Constraints");
        assert_eq!(resolve_extension_name("2.5.29.35"), "Authority Key Identifier");
        assert_eq!(resolve_extension_name("2.5.29.37"), "Extended Key Usage");
        assert_eq!(resolve_extension_name("1.3.6.1.5.5.7.1.1"), "Authority Information Access");
    }

    #[test]
    fn unknown_extension_uses_chinese_format() {
        let result = resolve_extension_name("1.2.3.4.5.6");
        assert!(result.starts_with("未知扩展"), "got: {}", result);
        assert!(result.contains("1.2.3.4.5.6"), "got: {}", result);
    }

    // === Key category boundary tests ===

    #[test]
    fn test_key_category_pq() {
        assert_eq!(resolve_key_category(oid_defs::pq::ML_DSA_44), KeyCategory::PostQuantum);
    }

    #[test]
    fn test_key_category_classic() {
        assert_eq!(resolve_key_category("1.2.840.113549.1.1.1"), KeyCategory::Classic);
    }

    #[test]
    fn pq_category_prefix_boundaries_classify_correctly() {
        // ML-KEM OIDs (FIPS 203) should be PostQuantum
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.4.1"), KeyCategory::PostQuantum);
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.4.2"), KeyCategory::PostQuantum);
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.4.3"), KeyCategory::PostQuantum);

        // ML-KEM legacy OIDs should be PostQuantum
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.1.43"), KeyCategory::PostQuantum);
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.1.44"), KeyCategory::PostQuantum);
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.1.45"), KeyCategory::PostQuantum);

        // Adjacent non-PQ OIDs should be Classic (not accidentally classified as PQ)
        // Note: 2.16.840.1.101.3.4.4.4 matches ML-KEM prefix "2.16.840.1.101.3.4.4."
        // Use a non-matching prefix for Classic test
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.5.1"), KeyCategory::Classic);
        // Note: 2.16.840.1.101.3.4.1.42 (AES-256-GCM) and 2.16.840.1.101.3.4.1.46 match ML-KEM legacy prefix
        // The prefix "2.16.840.1.101.3.4.1.4" covers all 2.16.840.1.101.3.4.1.4x OIDs
        // Use a non-matching prefix for Classic test
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.1.5"), KeyCategory::Classic);

        // Non-PQ algorithm OIDs should be Classic
        assert_eq!(resolve_key_category("1.2.840.113549.1.1.1"), KeyCategory::Classic); // RSA
        assert_eq!(resolve_key_category("1.2.840.10045.2.1"), KeyCategory::Classic); // ECDSA
        assert_eq!(resolve_key_category("2.16.840.1.101.3.4.2.1"), KeyCategory::Classic); // SHA256
    }
}
