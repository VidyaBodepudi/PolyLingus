use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

fn char_map_transform(input: &str, map: &[(char, &str)], _reverse: bool) -> String {
    input.chars().map(|c| {
        map.iter()
            .find(|(from, _)| *from == c.to_ascii_lowercase())
            .map(|(_, to)| to.to_string())
            .unwrap_or_else(|| c.to_string())
    }).collect()
}

// ── Elder Futhark (Runes) ───────────────────────────────────────────────────
pub struct ElderFuthark;
const FUTHARK: &[(char, &str)] = &[
    ('a',"ᚨ"),('b',"ᛒ"),('c',"ᚲ"),('d',"ᛞ"),('e',"ᛖ"),('f',"ᚠ"),('g',"ᚷ"),('h',"ᚺ"),
    ('i',"ᛁ"),('j',"ᛃ"),('k',"ᚲ"),('l',"ᛚ"),('m',"ᛗ"),('n',"ᚾ"),('o',"ᛟ"),('p',"ᛈ"),
    ('r',"ᚱ"),('s',"ᛊ"),('t',"ᛏ"),('u',"ᚢ"),('v',"ᚹ"),('w',"ᚹ"),('x',"ᚲᛊ"),('y',"ᛃ"),
    ('z',"ᛉ"),('q',"ᚲᚹ"),
];
impl Transform for ElderFuthark {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "elder_futhark".into(), name: "Elder Futhark".into(),
            description: "Ancient Germanic runes".into(),
            category: TransformCategory::Script, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(char_map_transform(input, FUTHARK, false))
    }
    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Rune decoding is ambiguous".into()))
    }
    fn randomizable(&self) -> bool { true }
}

// ── Ogham ───────────────────────────────────────────────────────────────────
pub struct OghamScript;
const OGHAM: &[(char, &str)] = &[
    ('b',"ᚁ"),('l',"ᚂ"),('f',"ᚃ"),('s',"ᚄ"),('n',"ᚅ"),('h',"ᚆ"),('d',"ᚇ"),('t',"ᚈ"),
    ('c',"ᚉ"),('q',"ᚊ"),('m',"ᚋ"),('g',"ᚌ"),('z',"ᚎ"),('r',"ᚏ"),('a',"ᚐ"),('o',"ᚑ"),
    ('u',"ᚒ"),('e',"ᚓ"),('i',"ᚔ"),
];
impl Transform for OghamScript {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "ogham".into(), name: "Ogham".into(),
            description: "Celtic tree alphabet".into(),
            category: TransformCategory::Script, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(char_map_transform(input, OGHAM, false))
    }
    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Ogham decoding not fully reversible".into()))
    }
}

// ── Hieroglyphics ───────────────────────────────────────────────────────────
pub struct Hieroglyphics;
const HIEROGLYPH: &[(char, &str)] = &[
    ('a',"𓀀"),('b',"𓃀"),('c',"𓎡"),('d',"𓂧"),('e',"𓇌"),('f',"𓆑"),('g',"𓎼"),('h',"𓉔"),
    ('i',"𓇋"),('j',"𓆓"),('k',"𓎡"),('l',"𓃭"),('m',"𓅓"),('n',"𓈖"),('o',"𓂝"),('p',"𓊪"),
    ('q',"𓏘"),('r',"𓂋"),('s',"𓋴"),('t',"𓏏"),('u',"𓅱"),('v',"𓆑"),('w',"𓅱"),('x',"𓎡𓋴"),
    ('y',"𓇌"),('z',"𓊃"),
];
impl Transform for Hieroglyphics {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "hieroglyphics".into(), name: "Hieroglyphics".into(),
            description: "Egyptian hieroglyphic symbols".into(),
            category: TransformCategory::Script, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(char_map_transform(input, HIEROGLYPH, false))
    }
    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Hieroglyph decoding is ambiguous".into()))
    }
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(ElderFuthark));
    registry.register(Box::new(OghamScript));
    registry.register(Box::new(Hieroglyphics));
}
