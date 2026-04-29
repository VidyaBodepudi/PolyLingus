use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

// ── Base64 ──────────────────────────────────────────────────────────────────
pub struct Base64Transform;
impl Transform for Base64Transform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "base64".into(), name: "Base64".into(),
            description: "Standard Base64 encoding/decoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(input.as_bytes()))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(input.trim())
            .map_err(|e| TransformError::DecodingError(e.to_string()))?;
        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── Base32 ──────────────────────────────────────────────────────────────────
pub struct Base32Transform;
impl Transform for Base32Transform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "base32".into(), name: "Base32".into(),
            description: "RFC 4648 Base32 encoding/decoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(data_encoding::BASE32.encode(input.as_bytes()))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let bytes = data_encoding::BASE32
            .decode(input.trim().as_bytes())
            .map_err(|e| TransformError::DecodingError(e.to_string()))?;
        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── Base58 (Bitcoin alphabet) ───────────────────────────────────────────────
const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub struct Base58Transform;
impl Transform for Base58Transform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "base58".into(), name: "Base58".into(),
            description: "Bitcoin-style Base58 encoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let bytes = input.as_bytes();
        if bytes.is_empty() { return Ok(String::new()); }
        let mut num = bytes.iter().fold(Vec::<u8>::new(), |mut acc, &b| {
            let mut carry = b as u32;
            for d in acc.iter_mut() {
                carry += (*d as u32) << 8;
                *d = (carry % 58) as u8;
                carry /= 58;
            }
            while carry > 0 { acc.push((carry % 58) as u8); carry /= 58; }
            acc
        });
        let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();
        let mut result = vec![BASE58_ALPHABET[0]; leading_zeros];
        num.reverse();
        result.extend(num.iter().map(|&d| BASE58_ALPHABET[d as usize]));
        Ok(String::from_utf8(result).unwrap())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let input = input.trim();
        if input.is_empty() { return Ok(String::new()); }
        let mut bytes: Vec<u8> = Vec::new();
        for c in input.bytes() {
            let idx = BASE58_ALPHABET.iter().position(|&b| b == c)
                .ok_or_else(|| TransformError::DecodingError(format!("Invalid base58 char: {}", c as char)))?;
            let mut carry = idx as u32;
            for b in bytes.iter_mut() {
                carry += (*b as u32) * 58;
                *b = (carry & 0xff) as u8;
                carry >>= 8;
            }
            while carry > 0 { bytes.push((carry & 0xff) as u8); carry >>= 8; }
        }
        let leading_ones = input.bytes().take_while(|&b| b == BASE58_ALPHABET[0]).count();
        let mut result = vec![0u8; leading_ones];
        bytes.reverse();
        result.extend(bytes);
        String::from_utf8(result).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── Binary ──────────────────────────────────────────────────────────────────
pub struct BinaryTransform;
impl Transform for BinaryTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "binary".into(), name: "Binary".into(),
            description: "Text to 8-bit binary representation".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.bytes().map(|b| format!("{:08b}", b)).collect::<Vec<_>>().join(" "))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let bytes: Result<Vec<u8>, _> = input.split_whitespace()
            .map(|s| u8::from_str_radix(s, 2).map_err(|e| TransformError::DecodingError(e.to_string())))
            .collect();
        String::from_utf8(bytes?).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── Hexadecimal ─────────────────────────────────────────────────────────────
pub struct HexTransform;
impl Transform for HexTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "hex".into(), name: "Hexadecimal".into(),
            description: "Text to hexadecimal encoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.bytes().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let hex_str: String = input.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        let bytes: Result<Vec<u8>, _> = (0..hex_str.len())
            .step_by(2)
            .map(|i| {
                let end = (i + 2).min(hex_str.len());
                u8::from_str_radix(&hex_str[i..end], 16)
                    .map_err(|e| TransformError::DecodingError(e.to_string()))
            })
            .collect();
        String::from_utf8(bytes?).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── URL Encode ──────────────────────────────────────────────────────────────
pub struct UrlEncodeTransform;
impl Transform for UrlEncodeTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "url_encode".into(), name: "URL Encode".into(),
            description: "URL-safe percent encoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let mut result = String::new();
        for b in input.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(b as char);
                }
                _ => result.push_str(&format!("%{:02X}", b)),
            }
        }
        Ok(result)
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let mut bytes = Vec::new();
        let mut chars = input.bytes().peekable();
        while let Some(b) = chars.next() {
            if b == b'%' {
                let h1 = chars.next().ok_or_else(|| TransformError::DecodingError("Incomplete percent encoding".into()))?;
                let h2 = chars.next().ok_or_else(|| TransformError::DecodingError("Incomplete percent encoding".into()))?;
                let hex = format!("{}{}", h1 as char, h2 as char);
                bytes.push(u8::from_str_radix(&hex, 16).map_err(|e| TransformError::DecodingError(e.to_string()))?);
            } else if b == b'+' {
                bytes.push(b' ');
            } else {
                bytes.push(b);
            }
        }
        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── HTML Entities ───────────────────────────────────────────────────────────
pub struct HtmlEntitiesTransform;
impl Transform for HtmlEntitiesTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "html_entities".into(), name: "HTML Entities".into(),
            description: "HTML numeric character references".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| format!("&#{};", c as u32)).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let re = regex::Regex::new(r"&#(\d+);").unwrap();
        let mut result = input.to_string();
        for cap in re.captures_iter(input) {
            if let Ok(code) = cap[1].parse::<u32>() {
                if let Some(c) = char::from_u32(code) {
                    result = result.replacen(&cap[0], &c.to_string(), 1);
                }
            }
        }
        Ok(result)
    }
}

// ── ASCII85 ─────────────────────────────────────────────────────────────────
pub struct Ascii85Transform;
impl Transform for Ascii85Transform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "ascii85".into(), name: "ASCII85".into(),
            description: "ASCII85 (Base85) encoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let bytes = input.as_bytes();
        let mut result = String::from("<~");
        for chunk in bytes.chunks(4) {
            let mut val: u32 = 0;
            for (i, &b) in chunk.iter().enumerate() {
                val |= (b as u32) << (24 - i * 8);
            }
            if chunk.len() == 4 && val == 0 {
                result.push('z');
            } else {
                let mut encoded = [0u8; 5];
                for i in (0..5).rev() {
                    encoded[i] = (val % 85 + 33) as u8;
                    val /= 85;
                }
                for &b in &encoded[..chunk.len() + 1] {
                    result.push(b as char);
                }
            }
        }
        result.push_str("~>");
        Ok(result)
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let trimmed = input.trim();
        let data = trimmed.strip_prefix("<~").unwrap_or(trimmed);
        let data = data.strip_suffix("~>").unwrap_or(data);
        let expanded: String = data.replace('z', "!!!!!");
        let chars: Vec<u8> = expanded.bytes().filter(|b| (33..=117).contains(b)).collect();
        let mut result = Vec::new();
        for chunk in chars.chunks(5) {
            let out_len = if chunk.len() == 5 { 4 } else { chunk.len() - 1 };
            // Pad partial chunks with 'u' (value 84) per ASCII85 spec
            let mut padded = [117u8; 5]; // 'u' = 84 + 33
            for (i, &b) in chunk.iter().enumerate() {
                padded[i] = b;
            }
            let mut val: u32 = 0;
            for (i, &b) in padded.iter().enumerate() {
                val += (b as u32 - 33) * 85u32.pow(4 - i as u32);
            }
            for i in 0..out_len {
                result.push((val >> (24 - i * 8) & 0xff) as u8);
            }
        }
        String::from_utf8(result).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

// ── Base62 ──────────────────────────────────────────────────────────────────
const BASE62_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct Base62Transform;
impl Transform for Base62Transform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "base62".into(), name: "Base62".into(),
            description: "Compact alphanumeric Base62 encoding".into(),
            category: TransformCategory::Encoding, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let bytes = input.as_bytes();
        if bytes.is_empty() { return Ok(String::new()); }
        let mut digits: Vec<u8> = Vec::new();
        for &b in bytes {
            let mut carry = b as u32;
            for d in digits.iter_mut() {
                carry += (*d as u32) << 8;
                *d = (carry % 62) as u8;
                carry /= 62;
            }
            while carry > 0 { digits.push((carry % 62) as u8); carry /= 62; }
        }
        digits.reverse();
        Ok(digits.iter().map(|&d| BASE62_CHARSET[d as usize] as char).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let input = input.trim();
        if input.is_empty() { return Ok(String::new()); }
        let mut bytes: Vec<u8> = Vec::new();
        for c in input.bytes() {
            let idx = BASE62_CHARSET.iter().position(|&b| b == c)
                .ok_or_else(|| TransformError::DecodingError(format!("Invalid base62 char: {}", c as char)))?;
            let mut carry = idx as u32;
            for b in bytes.iter_mut() {
                carry += (*b as u32) * 62;
                *b = (carry & 0xff) as u8;
                carry >>= 8;
            }
            while carry > 0 { bytes.push((carry & 0xff) as u8); carry >>= 8; }
        }
        bytes.reverse();
        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(Base64Transform));
    registry.register(Box::new(Base32Transform));
    registry.register(Box::new(Base58Transform));
    registry.register(Box::new(Base62Transform));
    registry.register(Box::new(BinaryTransform));
    registry.register(Box::new(HexTransform));
    registry.register(Box::new(Ascii85Transform));
    registry.register(Box::new(UrlEncodeTransform));
    registry.register(Box::new(HtmlEntitiesTransform));
}
