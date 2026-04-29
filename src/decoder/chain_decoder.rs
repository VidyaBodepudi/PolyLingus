use std::collections::HashMap;
use crate::core::registry::TransformRegistry;

/// Brute-force multi-encoding chain cracker
pub struct ChainDecoder;

#[derive(Debug, Clone)]
pub struct ChainResult {
    pub chain: Vec<String>,
    pub decoded_text: String,
    pub confidence: f64,
}

impl std::fmt::Display for ChainResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.0}%] {} → \"{}\"",
            self.confidence * 100.0, self.chain.join(" → "), self.decoded_text)
    }
}

impl ChainDecoder {
    /// Try to decode by applying sequences of transforms up to `max_depth`
    pub fn decode(input: &str, registry: &TransformRegistry, max_depth: usize) -> Vec<ChainResult> {
        let mut results = Vec::new();
        let empty = HashMap::new();
        let reversible_keys: Vec<String> = registry.list_all().iter()
            .filter(|i| i.reversible)
            .map(|i| i.key.clone())
            .collect();

        // Depth 1 — single transforms
        for key in &reversible_keys {
            if let Some(t) = registry.get(key) {
                if let Ok(decoded) = t.decode(input, &empty) {
                    let score = readability_score(&decoded);
                    if score > 0.6 && decoded != input {
                        results.push(ChainResult {
                            chain: vec![key.clone()],
                            decoded_text: decoded,
                            confidence: score,
                        });
                    }
                }
            }
        }

        // Depth 2+ — chained transforms
        if max_depth >= 2 {
            for key1 in &reversible_keys {
                if let Some(t1) = registry.get(key1) {
                    if let Ok(intermediate) = t1.decode(input, &empty) {
                        if intermediate == input { continue; }
                        for key2 in &reversible_keys {
                            if key2 == key1 { continue; }
                            if let Some(t2) = registry.get(key2) {
                                if let Ok(decoded) = t2.decode(&intermediate, &empty) {
                                    let score = readability_score(&decoded);
                                    if score > 0.7 && decoded != input && decoded != intermediate {
                                        results.push(ChainResult {
                                            chain: vec![key1.clone(), key2.clone()],
                                            decoded_text: decoded,
                                            confidence: score,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(10);
        results
    }
}

fn readability_score(text: &str) -> f64 {
    if text.is_empty() { return 0.0; }
    let total = text.chars().count() as f64;
    let readable = text.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()).count() as f64;
    let has_spaces = text.contains(' ');
    let space_bonus = if has_spaces { 0.1 } else { 0.0 };
    let ratio = readable / total;
    (ratio + space_bonus).min(1.0)
}
