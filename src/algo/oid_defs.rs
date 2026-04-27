#![allow(dead_code)]

/// GM/T (Chinese National Standard) OIDs
pub mod gmt {
    pub const SM2: &str = "1.2.156.10197.1.301";
    pub const SM2_KEY_EXCHANGE: &str = "1.2.156.10197.1.301.1";
    pub const SM2_ENCRYPTION: &str = "1.2.156.10197.1.301.3";
    pub const SM3: &str = "1.2.156.10197.1.401";
    pub const SM3_KEYLESS: &str = "1.2.156.10197.1.401.1";
    pub const HMAC_SM3: &str = "1.2.156.10197.1.401.3.1";
    pub const SM2_SM3_SIG: &str = "1.2.156.10197.1.501";
    pub const SM4: &str = "1.2.156.10197.1.104";
}

/// Post-quantum algorithm OIDs
pub mod pq {
    // ML-DSA (Module-Lattice-Based Digital Signature Standard)
    pub const ML_DSA_44: &str = "2.16.840.1.101.3.4.3.17";
    pub const ML_DSA_65: &str = "2.16.840.1.101.3.4.3.18";
    pub const ML_DSA_87: &str = "2.16.840.1.101.3.4.3.19";

    // SLH-DSA (Stateless Hash-Based Digital Signature Standard)
    pub const SLH_DSA_SHA2_128S: &str = "2.16.840.1.101.3.4.3.20";
    pub const SLH_DSA_SHA2_128F: &str = "2.16.840.1.101.3.4.3.21";
    pub const SLH_DSA_SHA2_192S: &str = "2.16.840.1.101.3.4.3.22";
    pub const SLH_DSA_SHA2_192F: &str = "2.16.840.1.101.3.4.3.23";
    pub const SLH_DSA_SHA2_256S: &str = "2.16.840.1.101.3.4.3.24";
    pub const SLH_DSA_SHA2_256F: &str = "2.16.840.1.101.3.4.3.25";
    pub const SLH_DSA_SHAKE_128S: &str = "2.16.840.1.101.3.4.3.26";
    pub const SLH_DSA_SHAKE_128F: &str = "2.16.840.1.101.3.4.3.27";
    pub const SLH_DSA_SHAKE_192S: &str = "2.16.840.1.101.3.4.3.28";
    pub const SLH_DSA_SHAKE_192F: &str = "2.16.840.1.101.3.4.3.29";
    pub const SLH_DSA_SHAKE_256S: &str = "2.16.840.1.101.3.4.3.30";
    pub const SLH_DSA_SHAKE_256F: &str = "2.16.840.1.101.3.4.3.31";

    // FN-DSA (Provisional IBM arc)
    pub const FN_DSA_512: &str = "1.3.6.1.4.1.2.267.8.1";
    pub const FN_DSA_1024: &str = "1.3.6.1.4.1.2.267.8.2";

    // FN-DSA FIPS 206 (Predicted NIST CSOR OIDs, not yet officially assigned)
    pub const FN_DSA_512_FIPS: &str = "2.16.840.1.101.3.4.3.47";
    pub const FN_DSA_1024_FIPS: &str = "2.16.840.1.101.3.4.3.48";

    // ML-KEM (FIPS 203)
    pub const ML_KEM_512: &str = "2.16.840.1.101.3.4.4.1";
    pub const ML_KEM_768: &str = "2.16.840.1.101.3.4.4.2";
    pub const ML_KEM_1024: &str = "2.16.840.1.101.3.4.4.3";

    // HQC (Provisional)
    pub const HQC_128: &str = "1.3.6.1.4.1.22554.5.1.1";
    pub const HQC_192: &str = "1.3.6.1.4.1.22554.5.1.2";
    pub const HQC_256: &str = "1.3.6.1.4.1.22554.5.1.3";
}
