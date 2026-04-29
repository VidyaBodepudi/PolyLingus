use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::pipeline::Pipeline;
use crate::decoder::{UniversalDecoder, ChainDecoder};
use crate::analysis::report::ReportGenerator;
use crate::payload::{Randomizer, PayloadFragmenter};
use crate::tokenizer::glitch_tokens;

/// Initialize the registry once and store it — WASM is single-threaded
fn get_registry() -> TransformRegistry {
    TransformRegistry::with_defaults()
}

/// List all available transforms as JSON
#[wasm_bindgen]
pub fn list_transforms() -> String {
    let registry = get_registry();
    let infos: Vec<serde_json::Value> = registry.list_all().iter().map(|info| {
        serde_json::json!({
            "key": info.key,
            "name": info.name,
            "description": info.description,
            "category": info.category.to_string(),
            "reversible": info.reversible,
            "parameters": info.parameters.iter().map(|p| {
                serde_json::json!({
                    "name": p.name,
                    "description": p.description,
                    "default_value": p.default_value,
                    "param_type": format!("{:?}", p.param_type),
                })
            }).collect::<Vec<_>>(),
        })
    }).collect();
    serde_json::to_string(&infos).unwrap_or_default()
}

/// Encode text using a specific transform
#[wasm_bindgen]
pub fn encode(key: &str, input: &str, params_json: &str) -> String {
    let registry = get_registry();
    let params: HashMap<String, String> = serde_json::from_str(params_json).unwrap_or_default();
    match registry.get(key) {
        Some(t) => match t.encode(input, &params) {
            Ok(result) => serde_json::json!({"ok": true, "result": result}).to_string(),
            Err(e) => serde_json::json!({"ok": false, "error": e.to_string()}).to_string(),
        },
        None => serde_json::json!({"ok": false, "error": format!("Unknown transform: {}", key)}).to_string(),
    }
}

/// Decode text using a specific transform
#[wasm_bindgen]
pub fn decode(key: &str, input: &str, params_json: &str) -> String {
    let registry = get_registry();
    let params: HashMap<String, String> = serde_json::from_str(params_json).unwrap_or_default();
    match registry.get(key) {
        Some(t) => match t.decode(input, &params) {
            Ok(result) => serde_json::json!({"ok": true, "result": result}).to_string(),
            Err(e) => serde_json::json!({"ok": false, "error": e.to_string()}).to_string(),
        },
        None => serde_json::json!({"ok": false, "error": format!("Unknown transform: {}", key)}).to_string(),
    }
}

/// Run a pipeline chain
#[wasm_bindgen]
pub fn run_pipeline(chain: &str, input: &str, reverse: bool) -> String {
    let registry = get_registry();
    let pipeline = Pipeline::from_chain_str("web_pipeline", chain);
    let result = if reverse { pipeline.decode(input, &registry) } else { pipeline.encode(input, &registry) };
    match result {
        Ok(output) => serde_json::json!({"ok": true, "result": output}).to_string(),
        Err(e) => serde_json::json!({"ok": false, "error": e.to_string()}).to_string(),
    }
}

/// Run full analysis and return JSON report
#[wasm_bindgen]
pub fn analyze_all(input: &str) -> String {
    ReportGenerator::to_json(input).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
}

/// Universal decode — try all transforms
#[wasm_bindgen]
pub fn universal_decode(input: &str) -> String {
    let registry = get_registry();
    let results = UniversalDecoder::decode(input, &registry);
    let out: Vec<serde_json::Value> = results.iter().take(10).map(|r| {
        serde_json::json!({"transform": r.transform_name, "key": r.transform_key, "decoded": r.decoded_text, "confidence": r.confidence})
    }).collect();
    serde_json::to_string(&out).unwrap_or_default()
}

/// Chain decode — brute-force multi-transform
#[wasm_bindgen]
pub fn chain_decode(input: &str, depth: usize) -> String {
    let registry = get_registry();
    let results = ChainDecoder::decode(input, &registry, depth);
    let out: Vec<serde_json::Value> = results.iter().take(10).map(|r| {
        serde_json::json!({"chain": r.chain, "decoded": r.decoded_text, "confidence": r.confidence})
    }).collect();
    serde_json::to_string(&out).unwrap_or_default()
}

/// Estimate token counts
#[wasm_bindgen]
pub fn estimate_tokens(input: &str) -> String {
    let est = glitch_tokens::estimate_tokens(input);
    serde_json::json!({
        "chars": est.char_count, "words": est.word_count,
        "gpt4": est.gpt4_estimate, "claude": est.claude_estimate, "llama": est.llama_estimate,
    }).to_string()
}

/// Fragment a payload
#[wasm_bindgen]
pub fn fragment_payload(input: &str, parts: usize, interleave: bool) -> String {
    let fragments = PayloadFragmenter::fragment(input, parts, None);
    if interleave {
        PayloadFragmenter::interleave(&fragments)
    } else {
        fragments.iter().map(|f| format!("{} {}", f.marker, f.content)).collect::<Vec<_>>().join("\n")
    }
}

/// Randomize text
#[wasm_bindgen]
pub fn randomize(input: &str) -> String {
    let registry = get_registry();
    Randomizer::randomize(input, &registry)
}

/// Image stego — embed message into raw RGBA pixels
#[wasm_bindgen]
pub fn image_stego_embed(pixels: &[u8], width: u32, height: u32, message: &str) -> Vec<u8> {
    let msg_bytes = message.as_bytes();
    let len = msg_bytes.len() as u32;
    let mut payload = Vec::with_capacity(4 + msg_bytes.len());
    payload.extend_from_slice(&len.to_be_bytes());
    payload.extend_from_slice(msg_bytes);

    let mut out = pixels.to_vec();
    let mut bit_idx = 0usize;

    for y in 0..height {
        for x in 0..width {
            let base = ((y * width + x) * 4) as usize;
            for ch in 0..3usize {
                if bit_idx >= payload.len() * 8 { return out; }
                let byte_idx = bit_idx / 8;
                let bit_pos = 7 - (bit_idx % 8);
                let bit = (payload[byte_idx] >> bit_pos) & 1;
                out[base + ch] = (out[base + ch] & 0xFE) | bit;
                bit_idx += 1;
            }
        }
    }
    out
}

/// Image stego — extract message from raw RGBA pixels
#[wasm_bindgen]
pub fn image_stego_extract(pixels: &[u8], width: u32, height: u32) -> String {
    let mut bits: Vec<u8> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let base = ((y * width + x) * 4) as usize;
            for ch in 0..3usize {
                if base + ch < pixels.len() {
                    bits.push(pixels[base + ch] & 1);
                }
                if bits.len() >= 32 + 8 * 65536 { break; }
            }
        }
    }
    if bits.len() < 32 { return String::from("Error: image too small"); }
    let mut len: u32 = 0;
    for i in 0..32 { len = (len << 1) | bits[i] as u32; }
    if len as usize > (bits.len() / 8).saturating_sub(4) {
        return String::from("Error: invalid message length");
    }
    let mut bytes = Vec::with_capacity(len as usize);
    for i in 0..len as usize {
        let mut byte: u8 = 0;
        for bit in 0..8 {
            let idx = 32 + i * 8 + bit;
            if idx < bits.len() { byte = (byte << 1) | bits[idx]; }
        }
        bytes.push(byte);
    }
    String::from_utf8(bytes).unwrap_or_else(|_| "Error: invalid UTF-8".into())
}
