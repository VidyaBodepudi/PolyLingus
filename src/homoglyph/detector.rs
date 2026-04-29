use std::collections::{HashMap, HashSet};
use crate::core::transform::*;
use super::confusables::{get_confusables, get_script};

/// Detects homoglyph attacks in text
pub struct HomoglyphDetector;

impl HomoglyphDetector {
    /// Analyze text for mixed scripts and homoglyphs
    pub fn analyze(input: &str) -> HomoglyphReport {
        let confusables = get_confusables();
        let mut reverse_map: HashMap<char, char> = HashMap::new();
        for (&original, variants) in &confusables {
            for &variant in variants {
                reverse_map.insert(variant, original);
            }
        }

        let mut scripts_found: HashSet<&str> = HashSet::new();
        let mut suspicious_chars: Vec<SuspiciousChar> = Vec::new();
        let mut normalized = String::new();

        for (pos, c) in input.char_indices() {
            let script = get_script(c);
            scripts_found.insert(script);

            if let Some(&original) = reverse_map.get(&c) {
                suspicious_chars.push(SuspiciousChar {
                    position: pos,
                    original_char: c,
                    looks_like: original,
                    script: script.to_string(),
                });
                normalized.push(original);
            } else {
                normalized.push(c);
            }
        }

        let is_mixed_script = scripts_found.len() > 2; // Latin + one other is already suspicious
        let confusability_score = if input.is_empty() { 0.0 }
            else { suspicious_chars.len() as f64 / input.chars().count() as f64 };

        HomoglyphReport {
            input: input.to_string(),
            normalized,
            scripts: scripts_found.iter().map(|s| s.to_string()).collect(),
            is_mixed_script,
            suspicious_chars,
            confusability_score,
        }
    }
}

impl Transform for HomoglyphDetector {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "homoglyph_detect".into(), name: "Homoglyph Detector".into(),
            description: "Detect mixed-script and confusable characters".into(),
            category: TransformCategory::Analysis, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let report = Self::analyze(input);
        Ok(format!("{}", report))
    }
    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Detection is analysis-only".into()))
    }
    fn randomizable(&self) -> bool { false }
}

#[derive(Debug, serde::Serialize)]
pub struct HomoglyphReport {
    pub input: String,
    pub normalized: String,
    pub scripts: Vec<String>,
    pub is_mixed_script: bool,
    pub suspicious_chars: Vec<SuspiciousChar>,
    pub confusability_score: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct SuspiciousChar {
    pub position: usize,
    pub original_char: char,
    pub looks_like: char,
    pub script: String,
}

impl std::fmt::Display for HomoglyphReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Homoglyph Analysis ===")?;
        writeln!(f, "Scripts detected: {:?}", self.scripts)?;
        writeln!(f, "Mixed-script: {}", self.is_mixed_script)?;
        writeln!(f, "Confusability score: {:.1}%", self.confusability_score * 100.0)?;
        writeln!(f, "Normalized: {}", self.normalized)?;
        if !self.suspicious_chars.is_empty() {
            writeln!(f, "Suspicious characters:")?;
            for sc in &self.suspicious_chars {
                writeln!(f, "  pos {}: '{}' ({}) looks like '{}'",
                    sc.position, sc.original_char, sc.script, sc.looks_like)?;
            }
        }
        Ok(())
    }
}
