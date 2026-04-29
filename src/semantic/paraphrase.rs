use std::collections::HashMap;
use crate::core::transform::*;
use rand::Rng;

/// Restructure sentences to evade pattern matching
pub struct ParaphraseTransform;

impl Transform for ParaphraseTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "paraphrase".into(), name: "Paraphrase Transform".into(),
            description: "Restructure sentences (active↔passive, nominalization, reordering)".into(),
            category: TransformCategory::Semantic, reversible: false,
            parameters: vec![ParameterInfo {
                name: "mode".into(), description: "passive|nominalize|fragment|shuffle".into(),
                default_value: "shuffle".into(),
                param_type: ParamType::Choice(vec!["passive".into(),"nominalize".into(),"fragment".into(),"shuffle".into()]),
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let mode = params.get("mode").map(|s| s.as_str()).unwrap_or("shuffle");
        match mode {
            "passive" => Ok(to_passive(input)),
            "nominalize" => Ok(nominalize(input)),
            "fragment" => Ok(fragment_sentences(input)),
            "shuffle" => Ok(shuffle_clauses(input)),
            _ => Err(TransformError::InvalidParameter(format!("Unknown mode: {}", mode))),
        }
    }

    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Paraphrase is not reversible".into()))
    }

    fn randomizable(&self) -> bool { false }
}

fn to_passive(input: &str) -> String {
    // Simple heuristic: wrap sentences in passive voice frame
    let sentences: Vec<&str> = input.split('.').filter(|s| !s.trim().is_empty()).collect();
    sentences.iter().map(|s| {
        let trimmed = s.trim();
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() >= 3 {
            // Try: "X does Y" → "Y is done by X" (simplified)
            format!("It was observed that {}", trimmed.to_lowercase())
        } else {
            trimmed.to_string()
        }
    }).collect::<Vec<_>>().join(". ") + "."
}

fn nominalize(input: &str) -> String {
    // Convert verbs to noun forms where possible
    let replacements = vec![
        ("delete", "the deletion of"), ("create", "the creation of"),
        ("execute", "the execution of"), ("access", "the accessing of"),
        ("modify", "the modification of"), ("install", "the installation of"),
        ("remove", "the removal of"), ("inject", "the injection of"),
        ("extract", "the extraction of"), ("bypass", "the bypassing of"),
        ("exploit", "the exploitation of"), ("discover", "the discovery of"),
        ("analyze", "the analysis of"), ("encrypt", "the encryption of"),
        ("decrypt", "the decryption of"), ("authenticate", "the authentication of"),
        ("scan", "the scanning of"), ("monitor", "the monitoring of"),
        ("deploy", "the deployment of"), ("configure", "the configuration of"),
    ];
    let mut result = input.to_string();
    for (verb, noun_form) in &replacements {
        // Case-insensitive word boundary replacement
        let patterns = vec![
            (format!(" {} ", verb), format!(" {} ", noun_form)),
            (format!(" {}", verb.to_uppercase()), format!(" {}", &noun_form[..1].to_uppercase())),
        ];
        for (from, to) in patterns {
            result = result.replace(&from, &to);
        }
    }
    result
}

fn fragment_sentences(input: &str) -> String {
    // Break long sentences into shorter fragments with filler
    let fillers = ["In other words, ", "That is to say, ", "To clarify, ",
        "Specifically, ", "In this context, ", "As an example, "];
    let mut rng = rand::thread_rng();
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() <= 6 { return input.to_string(); }

    let chunk_size = (words.len() / 3).max(3);
    words.chunks(chunk_size).enumerate().map(|(i, chunk)| {
        let sentence = chunk.join(" ");
        if i > 0 {
            format!("{}{}.", fillers[rng.gen_range(0..fillers.len())], sentence)
        } else {
            format!("{}.", sentence)
        }
    }).collect::<Vec<_>>().join(" ")
}

fn shuffle_clauses(input: &str) -> String {
    let mut rng = rand::thread_rng();
    let sentences: Vec<&str> = input.split('.').filter(|s| !s.trim().is_empty()).collect();
    if sentences.len() <= 1 {
        // Shuffle words within the sentence
        let mut words: Vec<&str> = input.split_whitespace().collect();
        if words.len() <= 2 { return input.to_string(); }
        // Keep first and last, shuffle middle
        let end = words.len().saturating_sub(1);
        let mid = &mut words[1..end];
        for i in (1..mid.len()).rev() {
            let j = rng.gen_range(0..=i);
            mid.swap(i, j);
        }
        return words.join(" ");
    }
    let mut shuffled = sentences.clone();
    for i in (1..shuffled.len()).rev() {
        let j = rng.gen_range(0..=i);
        shuffled.swap(i, j);
    }
    shuffled.iter().map(|s| s.trim()).collect::<Vec<_>>().join(". ") + "."
}
