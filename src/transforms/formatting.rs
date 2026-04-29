use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

// ── Leetspeak ───────────────────────────────────────────────────────────────
pub struct Leetspeak;
impl Transform for Leetspeak {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "leetspeak".into(), name: "Leetspeak".into(),
            description: "1337 sp34k substitutions".into(),
            category: TransformCategory::Formatting, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| match c.to_ascii_lowercase() {
            'a'=>'4','e'=>'3','i'=>'1','o'=>'0','s'=>'5','t'=>'7','l'=>'1','b'=>'8',
            'g'=>'9',_ => c,
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| match c {
            '4'=>'a','3'=>'e','1'=>'i','0'=>'o','5'=>'s','7'=>'t','8'=>'b','9'=>'g',_ => c,
        }).collect())
    }
}

// ── Pig Latin ───────────────────────────────────────────────────────────────
pub struct PigLatin;
impl Transform for PigLatin {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "pig_latin".into(), name: "Pig Latin".into(),
            description: "Simple word transformation".into(),
            category: TransformCategory::Formatting, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.split_whitespace().map(|word| {
            let lower = word.to_lowercase();
            if lower.starts_with(|c: char| "aeiou".contains(c)) {
                format!("{}way", word)
            } else {
                let consonants: String = lower.chars().take_while(|c| !"aeiou".contains(*c)).collect();
                let rest: String = word.chars().skip(consonants.len()).collect();
                format!("{}{}ay", rest, consonants)
            }
        }).collect::<Vec<_>>().join(" "))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.split_whitespace().map(|word| {
            if word.ends_with("way") && word.len() > 3 {
                word[..word.len()-3].to_string()
            } else if word.ends_with("ay") && word.len() > 2 {
                let body = &word[..word.len()-2];
                // Find where the moved consonants start (at the end of body)
                let vowel_pos = body.rfind(|c: char| "aeiouAEIOU".contains(c));
                match vowel_pos {
                    Some(pos) => {
                        let consonants = &body[pos+1..];
                        let rest = &body[..pos+1];
                        format!("{}{}", consonants, rest)
                    }
                    None => word.to_string(),
                }
            } else { word.to_string() }
        }).collect::<Vec<_>>().join(" "))
    }
}

// ── Vaporwave ───────────────────────────────────────────────────────────────
pub struct Vaporwave;
impl Transform for Vaporwave {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "vaporwave".into(), name: "Vaporwave".into(),
            description: "A e s t h e t i c spacing".into(),
            category: TransformCategory::Formatting, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if c == ' ' { "  ".to_string() }
            else if (0x21..=0x7e).contains(&(c as u32)) {
                let fw = char::from_u32(c as u32 - 0x21 + 0xFF01).unwrap_or(c);
                format!("{} ", fw)
            } else { c.to_string() }
        }).collect::<String>().trim_end().to_string())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        // Double space = original word boundary, single space = character padding
        let words: Vec<&str> = input.split("  ").collect();
        let decoded_words: Vec<String> = words.iter().map(|word| {
            word.chars().filter(|c| *c != ' ' && *c != '\u{3000}').map(|c| {
                if (0xFF01..=0xFF5E).contains(&(c as u32)) {
                    char::from_u32(c as u32 - 0xFF01 + 0x21).unwrap_or(c)
                } else { c }
            }).collect::<String>()
        }).collect();
        Ok(decoded_words.join(" "))
    }
}

// ── Mirror Text ─────────────────────────────────────────────────────────────
pub struct MirrorText;
impl Transform for MirrorText {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "mirror".into(), name: "Mirror Text".into(),
            description: "Reversed text".into(),
            category: TransformCategory::Formatting, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().rev().collect())
    }
    fn decode(&self, input: &str, p: &HashMap<String, String>) -> TransformResult {
        self.encode(input, p)
    }
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(Leetspeak));
    registry.register(Box::new(PigLatin));
    registry.register(Box::new(Vaporwave));
    registry.register(Box::new(MirrorText));
}
