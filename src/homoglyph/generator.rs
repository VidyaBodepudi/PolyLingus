use std::collections::HashMap;
use crate::core::transform::*;
use super::confusables::get_confusables;
use rand::Rng;

/// Generates homoglyph-obfuscated text
pub struct HomoglyphGenerator;

impl Transform for HomoglyphGenerator {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "homoglyph".into(), name: "Homoglyph Generator".into(),
            description: "Replace characters with visually identical Unicode lookalikes".into(),
            category: TransformCategory::Homoglyph, reversible: true,
            parameters: vec![
                ParameterInfo { name: "mode".into(), description: "full|targeted|random".into(),
                    default_value: "full".into(), param_type: ParamType::Choice(vec!["full".into(), "targeted".into(), "random".into()]) },
                ParameterInfo { name: "words".into(), description: "Comma-separated words to target (for targeted mode)".into(),
                    default_value: "".into(), param_type: ParamType::Text },
                ParameterInfo { name: "ratio".into(), description: "Substitution ratio 0.0-1.0 (for random mode)".into(),
                    default_value: "0.5".into(), param_type: ParamType::Text },
            ],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let confusables = get_confusables();
        let mode = params.get("mode").map(|s| s.as_str()).unwrap_or("full");

        match mode {
            "full" => Ok(substitute_all(input, &confusables)),
            "targeted" => {
                let words: Vec<&str> = params.get("words")
                    .map(|w| w.split(',').collect())
                    .unwrap_or_default();
                Ok(substitute_targeted(input, &confusables, &words))
            }
            "random" => {
                let ratio = params.get("ratio").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.5);
                Ok(substitute_random(input, &confusables, ratio))
            }
            _ => Err(TransformError::InvalidParameter(format!("Unknown mode: {}", mode))),
        }
    }

    fn decode(&self, input: &str, _params: &HashMap<String, String>) -> TransformResult {
        let confusables = get_confusables();
        let mut reverse_map: HashMap<char, char> = HashMap::new();
        for (&original, variants) in &confusables {
            for &variant in variants {
                reverse_map.insert(variant, original);
            }
        }
        Ok(input.chars().map(|c| *reverse_map.get(&c).unwrap_or(&c)).collect())
    }
}

fn substitute_all(input: &str, confusables: &HashMap<char, Vec<char>>) -> String {
    let mut rng = rand::thread_rng();
    input.chars().map(|c| {
        if let Some(variants) = confusables.get(&c) {
            if variants.is_empty() { c }
            else { variants[rng.gen_range(0..variants.len())] }
        } else { c }
    }).collect()
}

fn substitute_targeted(input: &str, confusables: &HashMap<char, Vec<char>>, words: &[&str]) -> String {
    let mut result = input.to_string();
    let mut rng = rand::thread_rng();
    for word in words {
        let replaced: String = word.chars().map(|c| {
            if let Some(variants) = confusables.get(&c) {
                if variants.is_empty() { c }
                else { variants[rng.gen_range(0..variants.len())] }
            } else { c }
        }).collect();
        result = result.replace(*word, &replaced);
    }
    result
}

fn substitute_random(input: &str, confusables: &HashMap<char, Vec<char>>, ratio: f64) -> String {
    let mut rng = rand::thread_rng();
    input.chars().map(|c| {
        if rng.gen::<f64>() < ratio {
            if let Some(variants) = confusables.get(&c) {
                if variants.is_empty() { c }
                else { variants[rng.gen_range(0..variants.len())] }
            } else { c }
        } else { c }
    }).collect()
}
