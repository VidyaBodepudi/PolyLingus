use std::collections::HashMap;
use crate::core::registry::TransformRegistry;

/// Universal decoder — tries all known transforms to decode unknown input
pub struct UniversalDecoder;

impl UniversalDecoder {
    /// Try to decode input using all registered transforms
    pub fn decode(input: &str, registry: &TransformRegistry) -> Vec<DecodeResult> {
        let mut results = Vec::new();
        let empty_params = HashMap::new();

        for info in registry.list_all() {
            if !info.reversible { continue; }
            if let Some(transform) = registry.get(&info.key) {
                if let Ok(decoded) = transform.decode(input, &empty_params) {
                    // Basic heuristic: decoded text should look like readable text
                    let ascii_ratio = decoded.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace()).count() as f64
                        / decoded.len().max(1) as f64;
                    if ascii_ratio > 0.5 && decoded != input {
                        results.push(DecodeResult {
                            transform_key: info.key.clone(),
                            transform_name: info.name.clone(),
                            decoded_text: decoded,
                            confidence: ascii_ratio,
                        });
                    }
                }
            }
        }

        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

#[derive(Debug)]
pub struct DecodeResult {
    pub transform_key: String,
    pub transform_name: String,
    pub decoded_text: String,
    pub confidence: f64,
}

impl std::fmt::Display for DecodeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.0}% - {}] {}", self.confidence * 100.0, self.transform_name, self.decoded_text)
    }
}
