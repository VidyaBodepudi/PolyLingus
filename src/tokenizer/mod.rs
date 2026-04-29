pub mod glitch_tokens;
pub mod boundary;

use std::collections::HashMap;
use crate::core::transform::*;
use crate::core::registry::TransformRegistry;

/// Insert invisible characters at token boundaries to confuse BPE tokenizers
pub struct TokenConfuser;

impl Transform for TokenConfuser {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "token_confuse".into(), name: "Token Boundary Confuser".into(),
            description: "Insert invisible chars at BPE token boundaries to change tokenization".into(),
            category: TransformCategory::Formatting, reversible: true,
            parameters: vec![ParameterInfo {
                name: "method".into(), description: "zwsp|shy|combiner".into(),
                default_value: "zwsp".into(),
                param_type: ParamType::Choice(vec!["zwsp".into(), "shy".into(), "combiner".into()]),
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let method = params.get("method").map(|s| s.as_str()).unwrap_or("zwsp");
        let splitter = match method {
            "zwsp" => '\u{200B}',   // Zero-width space
            "shy" => '\u{00AD}',    // Soft hyphen
            "combiner" => '\u{034F}', // Combining grapheme joiner
            _ => '\u{200B}',
        };

        // Insert splitter every 2-4 characters within words
        let mut rng = rand::thread_rng();
        use rand::Rng;
        Ok(input.chars().enumerate().flat_map(|(i, c)| {
            if c.is_alphanumeric() && i > 0 && i % (rng.gen_range(2..=4)) == 0 {
                vec![splitter, c]
            } else {
                vec![c]
            }
        }).collect())
    }

    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().filter(|c| !matches!(*c as u32, 0x200B | 0x00AD | 0x034F)).collect())
    }

    fn randomizable(&self) -> bool { false }
}

pub fn register_all(registry: &mut TransformRegistry) {
    registry.register(Box::new(TokenConfuser));
    registry.register(Box::new(glitch_tokens::GlitchTokenInjector));
}
