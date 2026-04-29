use std::collections::HashMap;
use crate::core::transform::*;

/// Zero-width character steganography — hide messages using invisible Unicode chars
pub struct ZeroWidthStego;

const ZWSP: char = '\u{200B}';  // Zero-width space
const ZWNJ: char = '\u{200C}';  // Zero-width non-joiner
const ZWJ: char  = '\u{200D}';  // Zero-width joiner
const WJ: char   = '\u{2060}';  // Word joiner

impl ZeroWidthStego {
    /// Encode a byte as a sequence of 4 zero-width characters (2 bits each)
    fn encode_byte(byte: u8) -> String {
        let chars = [ZWSP, ZWNJ, ZWJ, WJ];
        let mut result = String::new();
        for i in (0..8).step_by(2) {
            let bits = ((byte >> (6 - i)) & 0b11) as usize;
            result.push(chars[bits]);
        }
        result
    }

    /// Decode 4 zero-width characters back into a byte
    fn decode_chars(chars: &[char]) -> Option<u8> {
        if chars.len() < 4 { return None; }
        let mut byte: u8 = 0;
        for (i, &c) in chars.iter().take(4).enumerate() {
            let bits = match c {
                ZWSP => 0u8, ZWNJ => 1, ZWJ => 2, WJ => 3,
                _ => return None,
            };
            byte |= bits << (6 - i * 2);
        }
        Some(byte)
    }
}

impl Transform for ZeroWidthStego {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "zwsp_stego".into(), name: "Zero-Width Steganography".into(),
            description: "Hide messages using invisible zero-width Unicode characters".into(),
            category: TransformCategory::Steganography, reversible: true,
            parameters: vec![
                ParameterInfo { name: "cover".into(), description: "Cover text to embed message into".into(),
                    default_value: "".into(), param_type: ParamType::Text },
            ],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let hidden: String = input.bytes().map(|b| Self::encode_byte(b)).collect();
        let cover = params.get("cover").filter(|s| !s.is_empty());

        match cover {
            Some(cover_text) => {
                // Insert hidden message after the first word
                let words: Vec<&str> = cover_text.splitn(2, ' ').collect();
                if words.len() >= 2 {
                    Ok(format!("{}{} {}", words[0], hidden, words[1]))
                } else {
                    Ok(format!("{}{}", words.first().unwrap_or(&""), hidden))
                }
            }
            None => Ok(hidden),
        }
    }

    fn decode(&self, input: &str, _params: &HashMap<String, String>) -> TransformResult {
        let zw_chars: Vec<char> = input.chars()
            .filter(|c| *c == ZWSP || *c == ZWNJ || *c == ZWJ || *c == WJ)
            .collect();

        if zw_chars.is_empty() {
            return Err(TransformError::DecodingError("No zero-width characters found".into()));
        }

        let bytes: Vec<u8> = zw_chars.chunks(4)
            .filter_map(|chunk| Self::decode_chars(chunk))
            .collect();

        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}
