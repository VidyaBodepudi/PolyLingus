use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

// ── Affine Cipher ───────────────────────────────────────────────────────────
pub struct AffineCipher;
impl AffineCipher {
    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        for i in 1..m { if (a * i) % m == 1 { return Some(i); } }
        None
    }
}
impl Transform for AffineCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "affine".into(), name: "Affine Cipher".into(),
            description: "Mathematical cipher (ax + b) mod 26".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![
                ParameterInfo { name: "a".into(), description: "Multiplier (coprime to 26)".into(),
                    default_value: "5".into(), param_type: ParamType::Integer { min: 1, max: 25 } },
                ParameterInfo { name: "b".into(), description: "Shift".into(),
                    default_value: "8".into(), param_type: ParamType::Integer { min: 0, max: 25 } },
            ],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let a = params.get("a").and_then(|s| s.parse::<i32>().ok()).unwrap_or(5);
        let b = params.get("b").and_then(|s| s.parse::<i32>().ok()).unwrap_or(8);
        Ok(input.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let x = (c as u8 - base) as i32;
                let enc = ((a * x + b) % 26) as u8;
                (base + enc) as char
            } else { c }
        }).collect())
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let a = params.get("a").and_then(|s| s.parse::<i32>().ok()).unwrap_or(5);
        let b = params.get("b").and_then(|s| s.parse::<i32>().ok()).unwrap_or(8);
        let a_inv = Self::mod_inverse(a, 26)
            .ok_or_else(|| TransformError::InvalidParameter("a must be coprime to 26".into()))?;
        Ok(input.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let y = (c as u8 - base) as i32;
                let dec = ((a_inv * (y - b + 26)) % 26) as u8;
                (base + dec) as char
            } else { c }
        }).collect())
    }
}

// ── Atbash Cipher ───────────────────────────────────────────────────────────
pub struct AtbashCipher;
impl Transform for AtbashCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "atbash".into(), name: "Atbash Cipher".into(),
            description: "Mirror alphabet substitution (A↔Z, B↔Y, ...)".into(),
            category: TransformCategory::Cipher, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if c.is_ascii_uppercase() { (b'Z' - (c as u8 - b'A')) as char }
            else if c.is_ascii_lowercase() { (b'z' - (c as u8 - b'a')) as char }
            else { c }
        }).collect())
    }
    fn decode(&self, input: &str, p: &HashMap<String, String>) -> TransformResult {
        self.encode(input, p) // Atbash is self-inverse
    }
}

// ── Columnar Transposition ──────────────────────────────────────────────────
pub struct ColumnarTransposition;
impl Transform for ColumnarTransposition {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "columnar".into(), name: "Columnar Transposition".into(),
            description: "Key-based column reordering cipher".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![ParameterInfo {
                name: "key".into(), description: "Keyword for column ordering".into(),
                default_value: "ZEBRA".into(), param_type: ParamType::Text,
            }],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key = params.get("key").unwrap_or(&"ZEBRA".to_string()).to_uppercase();
        let key_order = get_key_order(&key);
        let cols = key.len();
        let chars: Vec<char> = input.chars().collect();
        let rows = (chars.len() + cols - 1) / cols;
        let mut grid = vec![vec![' '; cols]; rows];
        for (i, &c) in chars.iter().enumerate() {
            grid[i / cols][i % cols] = c;
        }
        let mut result = String::new();
        for &col in &key_order {
            for row in &grid { result.push(row[col]); }
        }
        Ok(result)
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key = params.get("key").unwrap_or(&"ZEBRA".to_string()).to_uppercase();
        let key_order = get_key_order(&key);
        let cols = key.len();
        let chars: Vec<char> = input.chars().collect();
        let rows = (chars.len() + cols - 1) / cols;
        let mut grid = vec![vec![' '; cols]; rows];
        let mut idx = 0;
        for &col in &key_order {
            for row in 0..rows {
                if idx < chars.len() { grid[row][col] = chars[idx]; idx += 1; }
            }
        }
        Ok(grid.into_iter().flatten().collect::<String>().trim_end().to_string())
    }
}

fn get_key_order(key: &str) -> Vec<usize> {
    let mut indexed: Vec<(usize, char)> = key.chars().enumerate().collect();
    indexed.sort_by_key(|(_, c)| *c);
    let mut order = vec![0; key.len()];
    for (rank, (orig_idx, _)) in indexed.iter().enumerate() {
        order[rank] = *orig_idx;
    }
    order
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(AffineCipher));
    registry.register(Box::new(AtbashCipher));
    registry.register(Box::new(ColumnarTransposition));
}
