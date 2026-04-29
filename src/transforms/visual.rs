use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

fn map_transform(input: &str, map: &[(char, char)]) -> String {
    input.chars().map(|c| {
        map.iter().find(|(from, _)| *from == c.to_ascii_lowercase())
            .map(|(_, to)| if c.is_ascii_uppercase() { to.to_uppercase().next().unwrap_or(*to) } else { *to })
            .unwrap_or(c)
    }).collect()
}

fn reverse_map(input: &str, map: &[(char, char)]) -> String {
    input.chars().map(|c| {
        map.iter().find(|(_, to)| *to == c)
            .map(|(from, _)| *from)
            .unwrap_or(c)
    }).collect()
}

// ── Upside Down ─────────────────────────────────────────────────────────────
pub struct UpsideDown;
const UPSIDE_DOWN_MAP: &[(char, char)] = &[
    ('a','ɐ'),('b','q'),('c','ɔ'),('d','p'),('e','ǝ'),('f','ɟ'),('g','ƃ'),('h','ɥ'),
    ('i','ᴉ'),('j','ɾ'),('k','ʞ'),('l','l'),('m','ɯ'),('n','u'),('o','o'),('p','d'),
    ('q','b'),('r','ɹ'),('s','s'),('t','ʇ'),('u','n'),('v','ʌ'),('w','ʍ'),('x','x'),
    ('y','ʎ'),('z','z'),('!','¡'),('?','¿'),('.','˙'),(',','\''),
    ('(', ')'), (')', '('),
];
impl Transform for UpsideDown {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "upside_down".into(), name: "Upside Down".into(),
            description: "Flip text upside down".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let mapped = map_transform(input, UPSIDE_DOWN_MAP);
        Ok(mapped.chars().rev().collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let reversed: String = input.chars().rev().collect();
        Ok(reverse_map(&reversed, UPSIDE_DOWN_MAP))
    }
}

// ── Full Width ──────────────────────────────────────────────────────────────
pub struct FullWidth;
impl Transform for FullWidth {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "full_width".into(), name: "Full Width".into(),
            description: "Convert to full-width Unicode".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if c == ' ' { '\u{3000}' }
            else if (0x21..=0x7e).contains(&(c as u32)) {
                char::from_u32(c as u32 - 0x21 + 0xFF01).unwrap_or(c)
            } else { c }
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if c == '\u{3000}' { ' ' }
            else if (0xFF01..=0xFF5E).contains(&(c as u32)) {
                char::from_u32(c as u32 - 0xFF01 + 0x21).unwrap_or(c)
            } else { c }
        }).collect())
    }
}

// ── Small Caps ──────────────────────────────────────────────────────────────
pub struct SmallCaps;
const SMALL_CAPS_MAP: &[(char, char)] = &[
    ('a','ᴀ'),('b','ʙ'),('c','ᴄ'),('d','ᴅ'),('e','ᴇ'),('f','ꜰ'),('g','ɢ'),('h','ʜ'),
    ('i','ɪ'),('j','ᴊ'),('k','ᴋ'),('l','ʟ'),('m','ᴍ'),('n','ɴ'),('o','ᴏ'),('p','ᴘ'),
    ('q','ǫ'),('r','ʀ'),('s','ꜱ'),('t','ᴛ'),('u','ᴜ'),('v','ᴠ'),('w','ᴡ'),('x','x'),
    ('y','ʏ'),('z','ᴢ'),
];
impl Transform for SmallCaps {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "small_caps".into(), name: "Small Caps".into(),
            description: "Convert to small capital letters".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(map_transform(input, SMALL_CAPS_MAP))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(reverse_map(input, SMALL_CAPS_MAP))
    }
}

// ── Bubble Text ─────────────────────────────────────────────────────────────
pub struct BubbleText;
impl Transform for BubbleText {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "bubble".into(), name: "Bubble Text".into(),
            description: "Enclose letters in circles".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if c.is_ascii_uppercase() {
                char::from_u32(0x24B6 + (c as u32 - 'A' as u32)).unwrap_or(c)
            } else if c.is_ascii_lowercase() {
                char::from_u32(0x24D0 + (c as u32 - 'a' as u32)).unwrap_or(c)
            } else if c.is_ascii_digit() && c != '0' {
                char::from_u32(0x2460 + (c as u32 - '1' as u32)).unwrap_or(c)
            } else if c == '0' { '\u{24EA}' }
            else { c }
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            let cp = c as u32;
            if (0x24B6..=0x24CF).contains(&cp) { (b'A' + (cp - 0x24B6) as u8) as char }
            else if (0x24D0..=0x24E9).contains(&cp) { (b'a' + (cp - 0x24D0) as u8) as char }
            else if (0x2460..=0x2468).contains(&cp) { (b'1' + (cp - 0x2460) as u8) as char }
            else if cp == 0x24EA { '0' }
            else { c }
        }).collect())
    }
}

// ── Strikethrough ───────────────────────────────────────────────────────────
pub struct Strikethrough;
impl Transform for Strikethrough {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "strikethrough".into(), name: "Strikethrough".into(),
            description: "Add strikethrough via combining chars".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().flat_map(|c| {
            if c.is_whitespace() { vec![c] } else { vec![c, '\u{0336}'] }
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().filter(|&c| c != '\u{0336}').collect())
    }
}

// ── Underline ───────────────────────────────────────────────────────────────
pub struct Underline;
impl Transform for Underline {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "underline".into(), name: "Underline".into(),
            description: "Add underlines via combining chars".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().flat_map(|c| {
            if c.is_whitespace() { vec![c] } else { vec![c, '\u{0332}'] }
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().filter(|&c| c != '\u{0332}').collect())
    }
}

// ── Braille ─────────────────────────────────────────────────────────────────
pub struct BrailleTransform;
const BRAILLE_MAP: &[(char, char)] = &[
    ('a','⠁'),('b','⠃'),('c','⠉'),('d','⠙'),('e','⠑'),('f','⠋'),('g','⠛'),('h','⠓'),
    ('i','⠊'),('j','⠚'),('k','⠅'),('l','⠇'),('m','⠍'),('n','⠝'),('o','⠕'),('p','⠏'),
    ('q','⠟'),('r','⠗'),('s','⠎'),('t','⠞'),('u','⠥'),('v','⠧'),('w','⠺'),('x','⠭'),
    ('y','⠽'),('z','⠵'),('1','⠂'),('2','⠆'),('3','⠒'),('4','⠲'),('5','⠢'),
    ('6','⠖'),('7','⠶'),('8','⠦'),('9','⠔'),('0','⠴'),
];
impl Transform for BrailleTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "braille".into(), name: "Braille".into(),
            description: "Convert to Braille patterns".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(map_transform(input, BRAILLE_MAP))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(reverse_map(input, BRAILLE_MAP))
    }
}

// ── Zalgo ───────────────────────────────────────────────────────────────────
pub struct ZalgoTransform;
impl Transform for ZalgoTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "zalgo".into(), name: "Zalgo".into(),
            description: "Glitch text with combining marks".into(),
            category: TransformCategory::Visual, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let combining_above: Vec<char> = (0x0300u32..=0x036Fu32)
            .filter_map(char::from_u32).collect();
        let combining_below: Vec<char> = (0x0316u32..=0x0349u32)
            .filter_map(char::from_u32).collect();
        Ok(input.chars().flat_map(|c| {
            let mut chars = vec![c];
            if !c.is_whitespace() {
                let n_above = rng.gen_range(1..=5);
                let n_below = rng.gen_range(1..=3);
                for _ in 0..n_above {
                    chars.push(combining_above[rng.gen_range(0..combining_above.len())]);
                }
                for _ in 0..n_below {
                    chars.push(combining_below[rng.gen_range(0..combining_below.len())]);
                }
            }
            chars
        }).collect())
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().filter(|c| !('\u{0300}'..='\u{036F}').contains(c)
            && !('\u{0316}'..='\u{0349}').contains(c)).collect())
    }
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(UpsideDown));
    registry.register(Box::new(FullWidth));
    registry.register(Box::new(SmallCaps));
    registry.register(Box::new(BubbleText));
    registry.register(Box::new(Strikethrough));
    registry.register(Box::new(Underline));
    registry.register(Box::new(BrailleTransform));
    registry.register(Box::new(ZalgoTransform));
}
