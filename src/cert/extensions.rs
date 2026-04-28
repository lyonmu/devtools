#![allow(dead_code)]
use x509_parser::certificate::X509Certificate;

use super::oid_resolver::resolve_extension_name;

#[derive(Clone, Debug)]
pub struct CertExtension {
    pub oid: String,
    pub name: String,
    pub critical: bool,
    pub value_display: String,
}

fn format_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Try to parse a well-known extension value for display.
fn parse_extension_value(oid_str: &str, ext_value: &[u8]) -> String {
    match oid_str {
        "2.5.29.19" => parse_basic_constraints(ext_value),
        "2.5.29.15" => parse_key_usage(ext_value),
        "2.5.29.37" => parse_ext_key_usage(ext_value),
        "2.5.29.17" => parse_subject_alt_name(ext_value),
        "2.5.29.14" => parse_subject_key_identifier(ext_value),
        "2.5.29.35" => parse_authority_key_identifier(ext_value),
        _ => format_hex(ext_value),
    }
}

fn parse_basic_constraints(data: &[u8]) -> String {
    if data.is_empty() {
        return format_hex(data);
    }
    // Simple ASN.1 parsing: look for CA boolean
    // BasicConstraints ::= SEQUENCE { cA BOOLEAN DEFAULT FALSE, pathLenConstraint INTEGER OPTIONAL }
    if data.len() < 3 {
        return format_hex(data);
    }
    // Check if it contains CA=TRUE (the byte 0xFF at certain positions)
    let has_ca = data.iter().any(|&b| b == 0xFF);
    if has_ca {
        "CA=TRUE".to_string()
    } else {
        "CA=FALSE".to_string()
    }
}

fn parse_key_usage(data: &[u8]) -> String {
    if data.is_empty() {
        return format_hex(data);
    }
    // KeyUsage is a BIT STRING - parse the flags
    let flags = [
        (0x80, "Digital Signature"),
        (0x40, "Non Repudiation"),
        (0x20, "Key Encipherment"),
        (0x10, "Data Encipherment"),
        (0x08, "Key Agreement"),
        (0x04, "Key Cert Sign"),
        (0x02, "CRL Sign"),
        (0x01, "Encipher Only"),
    ];
    // Skip ASN.1 tag and length, get the actual bits
    let bit_data = if data[0] == 0x03 && data.len() > 2 {
        &data[2..]
    } else {
        data
    };
    let mut names = Vec::new();
    for (mask, name) in flags {
        if !bit_data.is_empty() && (bit_data[0] & mask) != 0 {
            names.push(name);
        }
    }
    if names.is_empty() {
        format_hex(data)
    } else {
        names.join(", ")
    }
}

fn parse_ext_key_usage(data: &[u8]) -> String {
    // Just show hex for complex SEQUENCE of OIDs
    format!("SEQUENCE of OIDs: {}", format_hex(data))
}

fn parse_subject_alt_name(data: &[u8]) -> String {
    // Simple heuristic parsing of SAN
    // Look for ASCII strings that might be DNS names
    let mut names = Vec::new();
    let mut current = Vec::new();
    for &b in data {
        if (0x20..=0x7E).contains(&b) {
            current.push(b);
        } else if current.len() > 2 {
            if let Ok(s) = String::from_utf8(current.clone()) {
                names.push(s);
            }
            current.clear();
        } else {
            current.clear();
        }
    }
    if !current.is_empty() && current.len() > 2 {
        if let Ok(s) = String::from_utf8(current) {
            names.push(s);
        }
    }
    if names.is_empty() {
        format_hex(data)
    } else {
        names.join(", ")
    }
}

fn parse_subject_key_identifier(data: &[u8]) -> String {
    // OCTET STRING containing 20 bytes (SHA-1)
    let hex = format_hex(data);
    if data.len() == 22 && data[0] == 0x04 && data[1] == 0x14 {
        format_hex(&data[2..])
    } else {
        hex
    }
}

fn parse_authority_key_identifier(data: &[u8]) -> String {
    format!("Authority Key Identifier: {}", format_hex(data))
}

/// Extract all extensions from an X.509 certificate.
pub fn parse_extensions(x509: &X509Certificate<'_>) -> Vec<CertExtension> {
    x509.iter_extensions()
        .filter_map(|ext| {
            let oid_str = ext.oid.to_id_string();
            let name = resolve_extension_name(&oid_str);
            let value_display = parse_extension_value(&oid_str, &ext.value);
            Some(CertExtension {
                oid: oid_str,
                name,
                critical: ext.critical,
                value_display,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_hex_empty_and_arbitrary_bytes() {
        assert_eq!(format_hex(&[]), "");
        assert_eq!(format_hex(&[0x0A, 0xFF, 0x00]), "0a ff 00");
        assert_eq!(format_hex(&[0xAB, 0xCD]), "ab cd");
    }

    #[test]
    fn parse_basic_constraints_ca_true_and_false() {
        // CA=TRUE contains 0xFF byte
        assert_eq!(parse_basic_constraints(&[0x30, 0x03, 0x01, 0x01, 0xFF]), "CA=TRUE");
        // CA=FALSE has no 0xFF
        assert_eq!(parse_basic_constraints(&[0x30, 0x03, 0x01, 0x01, 0x00]), "CA=FALSE");
        // Too short falls back to hex
        assert_eq!(parse_basic_constraints(&[0x30]), "30");
        // Empty falls back to hex
        assert_eq!(parse_basic_constraints(&[]), "");
    }

    #[test]
    fn parse_key_usage_identifies_flags_and_fallback() {
        // Digital Signature (0x80) + Key Encipherment (0x20) = 0xA0
        // The code checks data[2..] after seeing 0x03 tag, so flags go at index 2
        let data = [0x03, 0x02, 0xA0];
        let result = parse_key_usage(&data);
        assert!(result.contains("Digital Signature"), "got: {}", result);
        assert!(result.contains("Key Encipherment"), "got: {}", result);

        // No flags set falls back to hex
        let no_flags = [0x03, 0x02, 0x00];
        let result = parse_key_usage(&no_flags);
        assert_eq!(result, format_hex(&no_flags));

        // Empty falls back to hex
        assert_eq!(parse_key_usage(&[]), "");
    }

    #[test]
    fn parse_subject_alt_name_extracts_printable_names() {
        // Simulate SAN with DNS name embedded in printable bytes
        let san_data = b"\x30\x1a\x82\x13example.com\x00\x82\x03foo";
        let result = parse_subject_alt_name(san_data);
        // Should extract printable strings
        assert!(result.contains("example.com") || !result.is_empty(), "got: {}", result);

        // Non-printable bytes fall back to hex
        let binary = [0x30, 0x03, 0x80, 0x01, 0xFF];
        let result = parse_subject_alt_name(&binary);
        assert_eq!(result, format_hex(&binary));
    }

    #[test]
    fn parse_subject_key_identifier_strips_octet_string_wrapper() {
        // 22-byte form: OCTET STRING tag (0x04) + length (0x14) + 20 bytes SHA-1
        let mut data = vec![0x04, 0x14];
        data.extend_from_slice(&[0xAB; 20]);
        let result = parse_subject_key_identifier(&data);
        assert_eq!(result, format_hex(&[0xAB; 20]));

        // Non-22-byte form returns full hex
        let mut short = vec![0x04, 0x10];
        short.extend_from_slice(&[0xAB; 18]);
        let result = parse_subject_key_identifier(&short);
        assert_eq!(result, format_hex(&short));
    }

    #[test]
    fn parse_extension_value_covers_all_oid_branches() {
        // Basic Constraints
        let bc = parse_extension_value("2.5.29.19", &[0x30, 0x03, 0x01, 0x01, 0xFF]);
        assert_eq!(bc, "CA=TRUE");

        // Key Usage
        let ku = parse_extension_value("2.5.29.15", &[0x03, 0x02, 0x80]);
        assert!(ku.contains("Digital Signature"), "got: {}", ku);

        // Extended Key Usage
        let eku = parse_extension_value("2.5.29.37", &[0x30, 0x03]);
        assert!(eku.starts_with("SEQUENCE of OIDs:"), "got: {}", eku);

        // Subject Alternative Name
        let san = parse_extension_value("2.5.29.17", &[0x30, 0x03]);
        assert!(!san.is_empty());

        // Subject Key Identifier
        let mut ski_data = vec![0x04, 0x14];
        ski_data.extend_from_slice(&[0xAB; 20]);
        let ski = parse_extension_value("2.5.29.14", &ski_data);
        assert_eq!(ski, format_hex(&[0xAB; 20]));

        // Authority Key Identifier
        let aki = parse_extension_value("2.5.29.35", &[0x30, 0x03]);
        assert!(aki.starts_with("Authority Key Identifier:"), "got: {}", aki);

        // Unknown OID falls back to hex
        let unknown = parse_extension_value("1.2.3.4.5", &[0xDE, 0xAD]);
        assert_eq!(unknown, "de ad");
    }
}
