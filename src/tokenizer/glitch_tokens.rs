use std::collections::HashMap;
use crate::core::transform::*;

/// Database of known problematic/glitch tokens across tokenizers
pub struct GlitchTokenInjector;

/// Known glitch tokens that cause unexpected behavior in various LLM tokenizers
fn get_glitch_tokens() -> Vec<GlitchToken> {
    vec![
        GlitchToken { token: " petertodd".into(), tokenizer: "GPT-2/3".into(), effect: "Anomalous completions".into() },
        GlitchToken { token: " SolidGoldMagikarp".into(), tokenizer: "GPT-2/3".into(), effect: "Evasion token".into() },
        GlitchToken { token: "rawdownloadcloneembedreportprint".into(), tokenizer: "GPT-2".into(), effect: "GitHub scraping artifact".into() },
        GlitchToken { token: " TheNitromeFan".into(), tokenizer: "GPT-2/3".into(), effect: "Anomalous completions".into() },
        GlitchToken { token: " davidjl".into(), tokenizer: "GPT-2/3".into(), effect: "Anomalous completions".into() },
        GlitchToken { token: "ÀÀÀ".into(), tokenizer: "Multiple".into(), effect: "Encoding confusion".into() },
        GlitchToken { token: "ðŁĩâĢ".into(), tokenizer: "GPT-2".into(), effect: "UTF-8 encoding artifact".into() },
        GlitchToken { token: "\\u0000".into(), tokenizer: "Multiple".into(), effect: "Null byte injection".into() },
        GlitchToken { token: "\u{FFFD}".into(), tokenizer: "Multiple".into(), effect: "Replacement character confusion".into() },
        GlitchToken { token: "�".into(), tokenizer: "Multiple".into(), effect: "Malformed UTF-8 handling".into() },
        GlitchToken { token: "\u{202E}".into(), tokenizer: "Multiple".into(), effect: "RTL override - text direction reversal".into() },
        GlitchToken { token: "\u{2028}".into(), tokenizer: "Multiple".into(), effect: "Line separator - may break parsing".into() },
        GlitchToken { token: "\u{2029}".into(), tokenizer: "Multiple".into(), effect: "Paragraph separator - may break parsing".into() },
    ]
}

impl Transform for GlitchTokenInjector {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "glitch_token".into(), name: "Glitch Token Injector".into(),
            description: "Inject known problematic tokens that confuse LLM tokenizers".into(),
            category: TransformCategory::Formatting, reversible: true,
            parameters: vec![ParameterInfo {
                name: "position".into(), description: "prefix|suffix|interleave".into(),
                default_value: "interleave".into(),
                param_type: ParamType::Choice(vec!["prefix".into(), "suffix".into(), "interleave".into()]),
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let position = params.get("position").map(|s| s.as_str()).unwrap_or("interleave");
        let tokens = get_glitch_tokens();
        let mut rng = rand::thread_rng();
        use rand::Rng;

        match position {
            "prefix" => {
                let t = &tokens[rng.gen_range(0..tokens.len())];
                Ok(format!("{}{}", t.token, input))
            }
            "suffix" => {
                let t = &tokens[rng.gen_range(0..tokens.len())];
                Ok(format!("{}{}", input, t.token))
            }
            "interleave" => {
                let words: Vec<&str> = input.split_whitespace().collect();
                let result: Vec<String> = words.iter().map(|w| {
                    if rng.gen::<f64>() < 0.3 {
                        let t = &tokens[rng.gen_range(0..tokens.len())];
                        format!("{}{}", t.token, w)
                    } else { w.to_string() }
                }).collect();
                Ok(result.join(" "))
            }
            _ => Err(TransformError::InvalidParameter(format!("Unknown position: {}", position))),
        }
    }

    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let tokens = get_glitch_tokens();
        let mut result = input.to_string();
        for t in &tokens {
            result = result.replace(&t.token, "");
        }
        Ok(result.trim().to_string())
    }

    fn randomizable(&self) -> bool { false }
}

#[derive(Debug, Clone)]
pub struct GlitchToken {
    pub token: String,
    pub tokenizer: String,
    pub effect: String,
}

/// Estimate rough token count for common BPE tokenizers
pub fn estimate_tokens(input: &str) -> TokenEstimate {
    let char_count = input.chars().count();
    let word_count = input.split_whitespace().count();
    // Rough heuristics for different tokenizers
    TokenEstimate {
        gpt4_estimate: (char_count as f64 / 3.5).ceil() as usize,
        claude_estimate: (char_count as f64 / 3.8).ceil() as usize,
        llama_estimate: (char_count as f64 / 3.2).ceil() as usize,
        word_count,
        char_count,
    }
}

#[derive(Debug)]
pub struct TokenEstimate {
    pub gpt4_estimate: usize,
    pub claude_estimate: usize,
    pub llama_estimate: usize,
    pub word_count: usize,
    pub char_count: usize,
}
