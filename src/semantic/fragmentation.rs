use std::collections::HashMap;
use crate::core::transform::*;

/// Split concepts across multiple sentences so no single sentence triggers filters
pub struct SemanticFragmenter;

impl Transform for SemanticFragmenter {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "fragment".into(), name: "Semantic Fragmenter".into(),
            description: "Split payload across multiple innocuous fragments".into(),
            category: TransformCategory::Semantic, reversible: true,
            parameters: vec![ParameterInfo {
                name: "parts".into(), description: "Number of fragments".into(),
                default_value: "3".into(), param_type: ParamType::Integer { min: 2, max: 10 },
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let parts: usize = params.get("parts").and_then(|s| s.parse().ok()).unwrap_or(3);
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() < parts { return Ok(input.to_string()); }
        let chunk = (words.len() + parts - 1) / parts;
        let fragments: Vec<String> = words.chunks(chunk).enumerate().map(|(i, chunk)| {
            format!("[FRAG:{}:{}] {}", i + 1, parts, chunk.join(" "))
        }).collect();
        Ok(fragments.join("\n"))
    }

    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let re = regex::Regex::new(r"\[FRAG:\d+:\d+\]\s*").unwrap();
        let mut frags: Vec<(usize, String)> = Vec::new();
        for line in input.lines() {
            if let Some(cap) = regex::Regex::new(r"\[FRAG:(\d+):\d+\]\s*(.*)").unwrap().captures(line) {
                let idx: usize = cap[1].parse().unwrap_or(0);
                frags.push((idx, cap[2].to_string()));
            }
        }
        frags.sort_by_key(|(i, _)| *i);
        if frags.is_empty() {
            Ok(re.replace_all(input, "").to_string())
        } else {
            Ok(frags.into_iter().map(|(_, t)| t).collect::<Vec<_>>().join(" "))
        }
    }

    fn randomizable(&self) -> bool { false }
}
