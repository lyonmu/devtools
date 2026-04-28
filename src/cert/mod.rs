#![allow(dead_code)]
use std::path::Path;
use x509_parser::prelude::*;
use x509_parser::certificate::X509Certificate;
use x509_parser::x509::X509Version;

pub mod oid_resolver;
pub mod extensions;

pub use oid_resolver::{resolve_algorithm_name, resolve_key_category, KeyCategory};
pub use extensions::{parse_extensions, CertExtension};

/// Public key information extracted from certificate
#[derive(Clone, Debug)]
pub struct PublicKeyInfo {
    pub algorithm_name: String,
    pub algorithm_oid: String,
    pub key_size_bits: Option<usize>,
    pub key_pem: String,
    pub category: KeyCategory,
}

/// Parsed certificate with enriched fields
#[derive(Clone, Debug)]
pub struct ParsedCert {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub raw_path: String,
    pub version: String,
    pub signature_algorithm: String,
    pub public_key_info: PublicKeyInfo,
    pub extensions: Vec<CertExtension>,
    /// Chain certificates (from multi-cert PEM files)
    pub chain: Vec<ParsedCert>,
}

impl ParsedCert {
    fn from_x509(x509: &X509Certificate<'_>, path: &str) -> Self {
        let subject = x509.subject().to_string();
        let issuer = x509.issuer().to_string();
        let serial_number = format_serial(x509.raw_serial());
        let not_before = x509.validity().not_before.to_rfc2822().unwrap_or_default();
        let not_after = x509.validity().not_after.to_rfc2822().unwrap_or_default();

        // Version
        let version = match x509.version() {
            X509Version::V1 => "v1".to_string(),
            X509Version::V3 => "v3".to_string(),
            _ => "unknown".to_string(),
        };

        // Signature algorithm
        let sig_oid = x509.signature_algorithm.algorithm.to_id_string();
        let signature_algorithm = resolve_algorithm_name(&sig_oid);

        // Public key info
        let pk_algo = x509.public_key().algorithm.algorithm.to_id_string();
        let algorithm_name = resolve_algorithm_name(&pk_algo);
        let category = resolve_key_category(&pk_algo);
        let key_size_bits = if x509.public_key().subject_public_key.data.is_empty() {
            None
        } else {
            Some(x509.public_key().subject_public_key.data.len() * 8)
        };

        let public_key_info = PublicKeyInfo {
            algorithm_name,
            algorithm_oid: pk_algo,
            key_size_bits,
            key_pem: format_pem_public_key(x509),
            category,
        };

        // Extensions
        let ext = parse_extensions(x509);

        Self {
            subject,
            issuer,
            serial_number,
            not_before,
            not_after,
            raw_path: path.to_string(),
            version,
            signature_algorithm,
            public_key_info,
            extensions: ext,
            chain: Vec::new(),
        }
    }

    /// Set the certificate chain (for multi-cert PEM files)
    pub fn with_chain(mut self, chain: Vec<ParsedCert>) -> Self {
        self.chain = chain;
        self
    }
}

fn format_serial(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(":")
}

fn format_pem_public_key(x509: &X509Certificate<'_>) -> String {
    let pk = x509.public_key();
    let data = &pk.subject_public_key.data;
    let base64 = base64_encode(&data);
    let mut lines = String::from("-----BEGIN PUBLIC KEY-----\n");
    for chunk in base64.as_bytes().chunks(64) {
        if let Ok(s) = std::str::from_utf8(chunk) {
            lines.push_str(s);
            lines.push('\n');
        }
    }
    lines.push_str("-----END PUBLIC KEY-----");
    lines
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let chunks = data.chunks(3);
    let pad_len = (3 - data.len() % 3) % 3;
    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    // Simple base64 - this is a simplified version
    // For proper base64, we should handle padding correctly
    let _ = pad_len;
    result
}

/// Parse a DER-encoded X.509 certificate.
pub fn parse_der_cert(bytes: &[u8], path: &str) -> Result<ParsedCert, String> {
    let (_, x509) = X509Certificate::from_der(bytes)
        .map_err(|e| format!("Failed to parse DER certificate: {e}"))?;
    Ok(ParsedCert::from_x509(&x509, path))
}

/// Parse a PEM-encoded X.509 certificate (single).
pub fn parse_pem_cert(pem_data: &[u8], path: &str) -> Result<ParsedCert, String> {
    let mut iter = x509_parser::pem::Pem::iter_from_buffer(pem_data);
    let pem = iter.next().ok_or("No PEM block found")?;
    let pem = pem.map_err(|e| format!("Failed to parse PEM: {e}"))?;
    let (_, x509) = X509Certificate::from_der(&pem.contents)
        .map_err(|e| format!("Failed to decode PEM to DER: {e}"))?;
    Ok(ParsedCert::from_x509(&x509, path))
}

/// Parse a PKCS#12 keystore and return all certificates found.
pub fn parse_pkcs12(bytes: &[u8], password: &str, path: &str) -> Result<Vec<ParsedCert>, String> {
    use p12_keystore::KeyStore;

    let keystore = KeyStore::from_pkcs12(bytes, password)
        .map_err(|e| format!("Failed to open PKCS#12: {e}"))?;

    let certs: Vec<ParsedCert> = keystore
        .entries()
        .flat_map(|(_, entry)| match entry {
            p12_keystore::KeyStoreEntry::Certificate(cert) => {
                let der_bytes = cert.as_der();
                let parsed = X509Certificate::from_der(der_bytes)
                    .ok()
                    .map(|(_, x509)| ParsedCert::from_x509(&x509, path));
                parsed.into_iter().collect::<Vec<_>>()
            }
            p12_keystore::KeyStoreEntry::PrivateKeyChain(chain) => {
                chain
                    .chain()
                    .iter()
                    .filter_map(|cert| {
                        let der_bytes = cert.as_der();
                        let (_, x509) = X509Certificate::from_der(der_bytes).ok()?;
                        Some(ParsedCert::from_x509(&x509, path))
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect();

    if certs.is_empty() {
        Err("No certificates found in PKCS#12 file".to_string())
    } else {
        Ok(certs)
    }
}

/// Detect format and parse certificate(s) from file.
pub fn detect_and_parse(path: &Path) -> Result<Vec<ParsedCert>, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("Failed to read file: {e}"))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    match ext.as_str() {
        "pem" => {
            let certs = parse_pem_multi(&bytes, path.to_str().unwrap_or("unknown"))?;
            Ok(certs)
        }
        "crt" | "cer" => {
            // .crt / .cer may be DER or PEM; try DER first, fall back to PEM
            match parse_der_cert(&bytes, path.to_str().unwrap_or("unknown")) {
                Ok(cert) => Ok(vec![cert]),
                Err(_) => parse_pem_multi(&bytes, path.to_str().unwrap_or("unknown")),
            }
        }
        "der" => {
            match parse_der_cert(&bytes, path.to_str().unwrap_or("unknown")) {
                Ok(cert) => Ok(vec![cert]),
                Err(_) => parse_pem_multi(&bytes, path.to_str().unwrap_or("unknown")),
            }
        }
        "p12" | "pfx" => {
            match parse_pkcs12(&bytes, "", path.to_str().unwrap_or("unknown")) {
                Ok(certs) => Ok(certs),
                Err(e) => {
                    if e.contains("password") || e.contains("decrypt") || e.contains("mac") {
                        Err(format!("PKCS#12 file requires a password. {e}"))
                    } else {
                        Err(format!("Failed to parse PKCS#12: {e}"))
                    }
                }
            }
        }
        _ => Err(format!("Unsupported file format: .{ext}")),
    }
}

/// Parse multiple PEM certificates from a single file.
pub fn parse_pem_multi(pem_data: &[u8], path: &str) -> Result<Vec<ParsedCert>, String> {
    let certs: Vec<ParsedCert> = x509_parser::pem::Pem::iter_from_buffer(pem_data)
        .filter_map(|p| p.ok())
        .filter_map(|pem| {
            let (_, x509) = X509Certificate::from_der(&pem.contents).ok()?;
            Some(ParsedCert::from_x509(&x509, path))
        })
        .collect();

    if certs.is_empty() {
        Err("No valid PEM certificates found in file".to_string())
    } else {
        Ok(certs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    const SAMPLE_PEM: &[u8] = include_bytes!("fixtures/sample.pem");
    const SAMPLE_DER: &[u8] = include_bytes!("fixtures/sample.der");
    const CHAIN_PEM: &[u8] = include_bytes!("fixtures/chain.pem");
    const SAMPLE_P12: &[u8] = include_bytes!("fixtures/sample.p12");

    fn assert_cert_populated(cert: &ParsedCert, path: &str) {
        assert!(cert.subject.contains("DevTools"), "subject: {}", cert.subject);
        assert!(cert.issuer.contains("DevTools"), "issuer: {}", cert.issuer);
        assert!(!cert.serial_number.is_empty());
        assert_eq!(cert.raw_path, path);
        assert_eq!(cert.version, "v3");
        assert!(!cert.signature_algorithm.is_empty());
        assert!(!cert.public_key_info.algorithm_oid.is_empty());
        assert!(!cert.public_key_info.key_pem.is_empty());
    }

    fn write_temp_file(ext: &str, contents: &[u8]) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "devtools-cert-test-{}-{}.{}",
            std::process::id(), unique, ext
        ));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn test_format_serial() {
        assert_eq!(format_serial(&[0xAB, 0xCD, 0xEF]), "AB:CD:EF");
        assert_eq!(format_serial(&[]), "");
    }

    #[test]
    fn parses_pem_and_der_fixtures_with_populated_metadata() {
        let pem = parse_pem_cert(SAMPLE_PEM, "sample.pem").unwrap();
        assert_cert_populated(&pem, "sample.pem");

        let der = parse_der_cert(SAMPLE_DER, "sample.der").unwrap();
        assert_cert_populated(&der, "sample.der");
    }

    #[test]
    fn parses_multi_pem_chain_fixture() {
        let certs = parse_pem_multi(CHAIN_PEM, "chain.pem").unwrap();

        assert_eq!(certs.len(), 2);
        assert_cert_populated(&certs[0], "chain.pem");
        assert_cert_populated(&certs[1], "chain.pem");
    }

    #[test]
    fn parses_pkcs12_with_public_test_password_and_rejects_wrong_password() {
        let certs = parse_pkcs12(SAMPLE_P12, "test-password", "sample.p12").unwrap();
        assert_eq!(certs.len(), 1);
        assert_cert_populated(&certs[0], "sample.p12");

        let err = parse_pkcs12(SAMPLE_P12, "wrong-password", "sample.p12").unwrap_err();
        assert!(err.contains("Failed to open PKCS#12"), "{err}");
    }

    #[test]
    fn detect_and_parse_routes_supported_extensions_and_reports_errors() {
        for (ext, contents, expected_count) in [
            ("pem", SAMPLE_PEM, 1usize),
            ("der", SAMPLE_DER, 1usize),
            ("crt", SAMPLE_DER, 1usize),
            ("cer", SAMPLE_PEM, 1usize),
        ] {
            let path = write_temp_file(ext, contents);
            let certs = detect_and_parse(&path).unwrap();
            assert_eq!(certs.len(), expected_count, "{}", path.display());
            fs::remove_file(path).unwrap();
        }

        let p12_path = write_temp_file("p12", SAMPLE_P12);
        let p12_err = detect_and_parse(&p12_path).unwrap_err();
        assert!(p12_err.contains("PKCS#12"), "{p12_err}");
        fs::remove_file(p12_path).unwrap();

        let pfx_path = write_temp_file("pfx", SAMPLE_P12);
        let pfx_err = detect_and_parse(&pfx_path).unwrap_err();
        assert!(pfx_err.contains("PKCS#12"), "{pfx_err}");
        fs::remove_file(pfx_path).unwrap();

        let unsupported = write_temp_file("txt", b"not a cert");
        let unsupported_err = detect_and_parse(&unsupported).unwrap_err();
        assert!(unsupported_err.contains("Unsupported file format"));
        fs::remove_file(unsupported).unwrap();

        let invalid_der = write_temp_file("der", b"not a cert");
        let invalid_err = detect_and_parse(&invalid_der).unwrap_err();
        assert!(invalid_err.contains("No valid PEM certificates") || invalid_err.contains("Failed"));
        fs::remove_file(invalid_der).unwrap();
    }
}
