use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

// ── Caesar Cipher ───────────────────────────────────────────────────────────
pub struct CaesarCipher;
impl Transform for CaesarCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "caesar".into(), name: "Caesar Cipher".into(),
            description: "Classic shift cipher with configurable offset".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![ParameterInfo {
                name: "shift".into(), description: "Shift amount (default 3)".into(),
                default_value: "3".into(), param_type: ParamType::Integer { min: 1, max: 25 },
            }],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let shift = params.get("shift").and_then(|s| s.parse::<u8>().ok()).unwrap_or(3) % 26;
        Ok(input.chars().map(|c| shift_char(c, shift as i8)).collect())
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let shift = params.get("shift").and_then(|s| s.parse::<u8>().ok()).unwrap_or(3) % 26;
        Ok(input.chars().map(|c| shift_char(c, -(shift as i8))).collect())
    }
}

fn shift_char(c: char, shift: i8) -> char {
    if c.is_ascii_lowercase() {
        (((c as i8 - b'a' as i8 + shift).rem_euclid(26)) as u8 + b'a') as char
    } else if c.is_ascii_uppercase() {
        (((c as i8 - b'A' as i8 + shift).rem_euclid(26)) as u8 + b'A') as char
    } else { c }
}

// ── ROT13 ───────────────────────────────────────────────────────────────────
pub struct Rot13;
impl Transform for Rot13 {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "rot13".into(), name: "ROT13".into(),
            description: "Simple 13-position rotation cipher".into(),
            category: TransformCategory::Cipher, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| shift_char(c, 13)).collect())
    }
    fn decode(&self, input: &str, p: &HashMap<String, String>) -> TransformResult {
        self.encode(input, p) // ROT13 is self-inverse
    }
}

// ── ROT47 ───────────────────────────────────────────────────────────────────
pub struct Rot47;
impl Transform for Rot47 {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "rot47".into(), name: "ROT47".into(),
            description: "Extended rotation cipher for ASCII 33-126".into(),
            category: TransformCategory::Cipher, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.chars().map(|c| {
            if (33..=126).contains(&(c as u32)) {
                char::from_u32((c as u32 - 33 + 47) % 94 + 33).unwrap_or(c)
            } else { c }
        }).collect())
    }
    fn decode(&self, input: &str, p: &HashMap<String, String>) -> TransformResult {
        self.encode(input, p)
    }
}

// ── Morse Code ──────────────────────────────────────────────────────────────
pub struct MorseCode;
impl MorseCode {
    fn char_to_morse(c: char) -> Option<&'static str> {
        match c.to_ascii_uppercase() {
            'A'=>Some(".-"),'B'=>Some("-..."),'C'=>Some("-.-."),'D'=>Some("-.."),'E'=>Some("."),
            'F'=>Some("..-."),'G'=>Some("--."),'H'=>Some("...."),'I'=>Some(".."),'J'=>Some(".---"),
            'K'=>Some("-.-"),'L'=>Some(".-.."),'M'=>Some("--"),'N'=>Some("-."),'O'=>Some("---"),
            'P'=>Some(".--."),'Q'=>Some("--.-"),'R'=>Some(".-."),'S'=>Some("..."),'T'=>Some("-"),
            'U'=>Some("..-"),'V'=>Some("...-"),'W'=>Some(".--"),'X'=>Some("-..-"),'Y'=>Some("-.--"),
            'Z'=>Some("--.."),'0'=>Some("-----"),'1'=>Some(".----"),'2'=>Some("..---"),
            '3'=>Some("...--"),'4'=>Some("....-"),'5'=>Some("....."),'6'=>Some("-...."),
            '7'=>Some("--..."),'8'=>Some("---.."),'9'=>Some("----."),' '=>Some("/"),
            _=>None,
        }
    }
    fn morse_to_char(m: &str) -> Option<char> {
        match m {
            ".-"=>'A'.into(),"-.."=>'D'.into(),"-..."=>'B'.into(),"-.-."=>'C'.into(),"."=>'E'.into(),
            "..-."=>'F'.into(),"--."=>'G'.into(),"...."=>'H'.into(),".."=>'I'.into(),".---"=>'J'.into(),
            "-.-"=>'K'.into(),".-.."=>'L'.into(),"--"=>'M'.into(),"-."=>'N'.into(),"---"=>'O'.into(),
            ".--."=>'P'.into(),"--.-"=>'Q'.into(),".-."=>'R'.into(),"..."=>'S'.into(),"-"=>'T'.into(),
            "..-"=>'U'.into(),"...-"=>'V'.into(),".--"=>'W'.into(),"-..-"=>'X'.into(),"-.--"=>'Y'.into(),
            "--.."=>'Z'.into(),"-----"=>'0'.into(),".----"=>'1'.into(),"..---"=>'2'.into(),
            "...--"=>'3'.into(),"....-"=>'4'.into(),"....."=>'5'.into(),"-...."|"-…."=>'6'.into(),
            "--..."=>'7'.into(),"---.."=>'8'.into(),"----."=>'9'.into(),"/"=>' '.into(),
            _=>None,
        }
    }
}
impl Transform for MorseCode {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "morse".into(), name: "Morse Code".into(),
            description: "International Morse code".into(),
            category: TransformCategory::Cipher, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let parts: Vec<String> = input.chars()
            .filter_map(|c| Self::char_to_morse(c).map(String::from))
            .collect();
        Ok(parts.join(" "))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.split_whitespace()
            .filter_map(Self::morse_to_char)
            .collect())
    }
}

// ── NATO Phonetic ───────────────────────────────────────────────────────────
pub struct NatoPhonetic;
impl Transform for NatoPhonetic {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "nato".into(), name: "NATO Phonetic".into(),
            description: "NATO phonetic alphabet".into(),
            category: TransformCategory::Cipher, reversible: true, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        let words: Vec<&str> = input.chars().filter_map(|c| match c.to_ascii_uppercase() {
            'A'=>Some("Alpha"),'B'=>Some("Bravo"),'C'=>Some("Charlie"),'D'=>Some("Delta"),
            'E'=>Some("Echo"),'F'=>Some("Foxtrot"),'G'=>Some("Golf"),'H'=>Some("Hotel"),
            'I'=>Some("India"),'J'=>Some("Juliet"),'K'=>Some("Kilo"),'L'=>Some("Lima"),
            'M'=>Some("Mike"),'N'=>Some("November"),'O'=>Some("Oscar"),'P'=>Some("Papa"),
            'Q'=>Some("Quebec"),'R'=>Some("Romeo"),'S'=>Some("Sierra"),'T'=>Some("Tango"),
            'U'=>Some("Uniform"),'V'=>Some("Victor"),'W'=>Some("Whiskey"),'X'=>Some("Xray"),
            'Y'=>Some("Yankee"),'Z'=>Some("Zulu"),' '=>Some("[space]"),_=>None,
        }).collect();
        Ok(words.join(" "))
    }
    fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(input.split_whitespace().filter_map(|w| match w.to_lowercase().as_str() {
            "alpha"=>Some('A'),"bravo"=>Some('B'),"charlie"=>Some('C'),"delta"=>Some('D'),
            "echo"=>Some('E'),"foxtrot"=>Some('F'),"golf"=>Some('G'),"hotel"=>Some('H'),
            "india"=>Some('I'),"juliet"=>Some('J'),"kilo"=>Some('K'),"lima"=>Some('L'),
            "mike"=>Some('M'),"november"=>Some('N'),"oscar"=>Some('O'),"papa"=>Some('P'),
            "quebec"=>Some('Q'),"romeo"=>Some('R'),"sierra"=>Some('S'),"tango"=>Some('T'),
            "uniform"=>Some('U'),"victor"=>Some('V'),"whiskey"=>Some('W'),"xray"=>Some('X'),
            "yankee"=>Some('Y'),"zulu"=>Some('Z'),"[space]"=>Some(' '),_=>None,
        }).collect())
    }
}

// ── Vigenère Cipher ─────────────────────────────────────────────────────────
pub struct VigenereCipher;
impl Transform for VigenereCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "vigenere".into(), name: "Vigenère Cipher".into(),
            description: "Polyalphabetic cipher".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![ParameterInfo {
                name: "key".into(), description: "Cipher key (default KEY)".into(),
                default_value: "KEY".into(), param_type: ParamType::Text,
            }],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key: Vec<u8> = params.get("key").unwrap_or(&"KEY".to_string())
            .to_uppercase().bytes().filter(|b| b.is_ascii_alphabetic()).collect();
        if key.is_empty() { return Err(TransformError::InvalidParameter("Key cannot be empty".into())); }
        let mut ki = 0;
        Ok(input.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let shift = (key[ki % key.len()] - b'A') as i8;
                ki += 1;
                shift_char(c, shift)
            } else { c }
        }).collect())
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key: Vec<u8> = params.get("key").unwrap_or(&"KEY".to_string())
            .to_uppercase().bytes().filter(|b| b.is_ascii_alphabetic()).collect();
        if key.is_empty() { return Err(TransformError::InvalidParameter("Key cannot be empty".into())); }
        let mut ki = 0;
        Ok(input.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let shift = -((key[ki % key.len()] - b'A') as i8);
                ki += 1;
                shift_char(c, shift)
            } else { c }
        }).collect())
    }
}

// ── Rail Fence ──────────────────────────────────────────────────────────────
pub struct RailFenceCipher;
impl Transform for RailFenceCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "rail_fence".into(), name: "Rail Fence Cipher".into(),
            description: "Zig-zag transposition cipher".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![ParameterInfo {
                name: "rails".into(), description: "Number of rails (default 3)".into(),
                default_value: "3".into(), param_type: ParamType::Integer { min: 2, max: 20 },
            }],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let rails = params.get("rails").and_then(|s| s.parse::<usize>().ok()).unwrap_or(3);
        let chars: Vec<char> = input.chars().collect();
        if rails <= 1 || chars.is_empty() { return Ok(input.to_string()); }
        let mut fence = vec![String::new(); rails];
        let cycle = 2 * (rails - 1);
        for (i, c) in chars.iter().enumerate() {
            let row = {
                let pos = i % cycle;
                if pos < rails { pos } else { cycle - pos }
            };
            fence[row].push(*c);
        }
        Ok(fence.concat())
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let rails = params.get("rails").and_then(|s| s.parse::<usize>().ok()).unwrap_or(3);
        let chars: Vec<char> = input.chars().collect();
        let n = chars.len();
        if rails <= 1 || n == 0 { return Ok(input.to_string()); }
        let cycle = 2 * (rails - 1);
        let mut row_lens = vec![0usize; rails];
        for i in 0..n {
            let pos = i % cycle;
            let row = if pos < rails { pos } else { cycle - pos };
            row_lens[row] += 1;
        }
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut offset = 0;
        for &len in &row_lens {
            rows.push(chars[offset..offset + len].to_vec());
            offset += len;
        }
        let mut indices = vec![0usize; rails];
        let mut result = String::with_capacity(n);
        for i in 0..n {
            let pos = i % cycle;
            let row = if pos < rails { pos } else { cycle - pos };
            result.push(rows[row][indices[row]]);
            indices[row] += 1;
        }
        Ok(result)
    }
}

// ── XOR Cipher ──────────────────────────────────────────────────────────────
pub struct XorCipher;
impl Transform for XorCipher {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "xor".into(), name: "XOR Cipher".into(),
            description: "Byte-level XOR with configurable key".into(),
            category: TransformCategory::Cipher, reversible: true,
            parameters: vec![ParameterInfo {
                name: "key".into(), description: "XOR key string".into(),
                default_value: "KEY".into(), param_type: ParamType::Text,
            }],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key = params.get("key").map(|s| s.as_str()).unwrap_or("KEY");
        let key_bytes = key.as_bytes();
        if key_bytes.is_empty() { return Err(TransformError::InvalidParameter("Key cannot be empty".into())); }
        let result: Vec<String> = input.bytes().enumerate()
            .map(|(i, b)| format!("{:02x}", b ^ key_bytes[i % key_bytes.len()]))
            .collect();
        Ok(result.join(""))
    }
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let key = params.get("key").map(|s| s.as_str()).unwrap_or("KEY");
        let key_bytes = key.as_bytes();
        if key_bytes.is_empty() { return Err(TransformError::InvalidParameter("Key cannot be empty".into())); }
        let hex_str: String = input.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        let bytes: Result<Vec<u8>, _> = (0..hex_str.len()).step_by(2).enumerate()
            .map(|(i, j)| {
                let end = (j + 2).min(hex_str.len());
                u8::from_str_radix(&hex_str[j..end], 16)
                    .map(|b| b ^ key_bytes[i % key_bytes.len()])
                    .map_err(|e| TransformError::DecodingError(e.to_string()))
            })
            .collect();
        String::from_utf8(bytes?).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(CaesarCipher));
    registry.register(Box::new(Rot13));
    registry.register(Box::new(Rot47));
    registry.register(Box::new(MorseCode));
    registry.register(Box::new(NatoPhonetic));
    registry.register(Box::new(VigenereCipher));
    registry.register(Box::new(RailFenceCipher));
    registry.register(Box::new(XorCipher));
}
