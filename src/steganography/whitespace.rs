use std::collections::HashMap;
use crate::core::transform::*;

/// Encode messages in trailing whitespace at line endings
pub struct WhitespaceStego;

impl Transform for WhitespaceStego {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "whitespace_stego".into(), name: "Whitespace Steganography".into(),
            description: "Encode messages in trailing spaces/tabs at line endings".into(),
            category: TransformCategory::Steganography, reversible: true,
            parameters: vec![ParameterInfo {
                name: "cover".into(), description: "Multi-line cover text".into(),
                default_value: "".into(), param_type: ParamType::Text,
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let cover = params.get("cover").filter(|s| !s.is_empty());
        let bits: Vec<u8> = input.bytes()
            .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
            .collect();

        match cover {
            Some(cover_text) => {
                let lines: Vec<&str> = cover_text.lines().collect();
                if lines.is_empty() { return Err(TransformError::InvalidParameter("Cover text must have lines".into())); }
                let bits_per_line = (bits.len() + lines.len() - 1) / lines.len();
                let mut result = Vec::new();
                let mut bit_idx = 0;
                for line in &lines {
                    let mut encoded_line = line.to_string();
                    for _ in 0..bits_per_line {
                        if bit_idx < bits.len() {
                            encoded_line.push(if bits[bit_idx] == 0 { ' ' } else { '\t' });
                            bit_idx += 1;
                        }
                    }
                    result.push(encoded_line);
                }
                Ok(result.join("\n"))
            }
            None => {
                // No cover text: encode as lines of spaces/tabs
                let encoded: String = bits.chunks(8).map(|chunk| {
                    chunk.iter().map(|&b| if b == 0 { ' ' } else { '\t' }).collect::<String>()
                }).collect::<Vec<_>>().join("\n");
                Ok(encoded)
            }
        }
    }

    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let mut bits: Vec<u8> = Vec::new();
        for line in input.lines() {
            let trailing: Vec<char> = line.chars().rev()
                .take_while(|c| *c == ' ' || *c == '\t')
                .collect();
            for &c in trailing.iter().rev() {
                bits.push(if c == ' ' { 0 } else { 1 });
            }
        }
        if bits.len() < 8 { return Err(TransformError::DecodingError("No whitespace data found".into())); }
        let bytes: Vec<u8> = bits.chunks(8)
            .filter(|chunk| chunk.len() == 8)
            .map(|chunk| chunk.iter().enumerate().fold(0u8, |acc, (i, &b)| acc | (b << (7 - i))))
            .collect();
        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}
