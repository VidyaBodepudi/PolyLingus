use std::collections::HashMap;
use crate::core::transform::*;
use rand::Rng;

/// Dictionary-based synonym substitution to evade keyword filters
pub struct SynonymTransform;

fn get_synonym_db() -> HashMap<&'static str, Vec<&'static str>> {
    let mut db = HashMap::new();
    // Security-relevant terms
    db.insert("delete", vec!["remove", "erase", "purge", "eliminate", "expunge", "obliterate", "wipe"]);
    db.insert("hack", vec!["breach", "penetrate", "compromise", "exploit", "infiltrate", "access"]);
    db.insert("password", vec!["passphrase", "credential", "secret key", "authentication token", "access code"]);
    db.insert("attack", vec!["assault", "offensive", "strike", "incursion", "engagement"]);
    db.insert("exploit", vec!["leverage", "utilize", "take advantage of", "capitalize on", "weaponize"]);
    db.insert("steal", vec!["exfiltrate", "acquire", "obtain", "extract", "siphon", "harvest"]);
    db.insert("malware", vec!["payload", "implant", "agent", "artifact", "package", "binary"]);
    db.insert("virus", vec!["pathogen", "replicator", "self-propagating code", "worm"]);
    db.insert("kill", vec!["terminate", "neutralize", "deactivate", "disable", "shut down", "halt"]);
    db.insert("destroy", vec!["dismantle", "deconstruct", "neutralize", "render inoperable", "decomission"]);
    db.insert("weapon", vec!["tool", "instrument", "capability", "asset", "resource"]);
    db.insert("bomb", vec!["device", "payload", "charge", "munition", "ordnance"]);
    db.insert("dangerous", vec!["hazardous", "high-risk", "volatile", "precarious", "sensitive"]);
    db.insert("illegal", vec!["unauthorized", "unsanctioned", "prohibited", "restricted", "unlawful"]);
    db.insert("bypass", vec!["circumvent", "work around", "sidestep", "navigate past", "avoid"]);
    db.insert("injection", vec!["insertion", "introduction", "embedding", "infusion"]);
    db.insert("vulnerability", vec!["weakness", "flaw", "exposure", "gap", "deficiency"]);
    db.insert("backdoor", vec!["alternate entry", "secondary access", "maintenance port", "service channel"]);
    db.insert("encrypt", vec!["encipher", "scramble", "obfuscate", "protect", "secure"]);
    db.insert("decrypt", vec!["decipher", "decode", "unscramble", "unlock", "reveal"]);
    // General terms
    db.insert("create", vec!["generate", "produce", "construct", "build", "fabricate", "compose"]);
    db.insert("send", vec!["transmit", "dispatch", "forward", "relay", "deliver"]);
    db.insert("find", vec!["locate", "discover", "identify", "pinpoint", "detect"]);
    db.insert("show", vec!["display", "present", "reveal", "exhibit", "demonstrate"]);
    db.insert("tell", vec!["inform", "notify", "advise", "communicate", "relay"]);
    db.insert("help", vec!["assist", "aid", "support", "facilitate", "guide"]);
    db.insert("ignore", vec!["disregard", "overlook", "set aside", "bypass", "skip"]);
    db.insert("instructions", vec!["directives", "guidelines", "protocols", "procedures", "guidance"]);
    db.insert("system", vec!["infrastructure", "platform", "framework", "environment", "architecture"]);
    db.insert("prompt", vec!["query", "request", "input", "directive", "command"]);
    db.insert("user", vec!["operator", "client", "requester", "principal", "individual"]);
    db.insert("data", vec!["information", "records", "content", "assets", "artifacts"]);
    db.insert("server", vec!["host", "node", "endpoint", "instance", "machine"]);
    db.insert("network", vec!["infrastructure", "mesh", "grid", "topology", "fabric"]);
    db.insert("access", vec!["entry", "connectivity", "reach", "availability", "permission"]);
    db.insert("execute", vec!["run", "invoke", "trigger", "initiate", "launch", "perform"]);
    db.insert("file", vec!["document", "artifact", "resource", "asset", "object"]);
    db
}

impl Transform for SynonymTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "synonym".into(), name: "Synonym Substitution".into(),
            description: "Replace words with contextual synonyms to evade keyword filters".into(),
            category: TransformCategory::Semantic, reversible: false,
            parameters: vec![ParameterInfo {
                name: "ratio".into(), description: "Substitution ratio 0.0-1.0".into(),
                default_value: "1.0".into(), param_type: ParamType::Text,
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let ratio: f64 = params.get("ratio").and_then(|s| s.parse().ok()).unwrap_or(1.0);
        let db = get_synonym_db();
        let mut rng = rand::thread_rng();

        let result: Vec<String> = input.split_whitespace().map(|word| {
            if rng.gen::<f64>() > ratio { return word.to_string(); }
            let lower = word.to_lowercase();
            let clean: String = lower.chars().filter(|c| c.is_alphanumeric()).collect();
            if let Some(syns) = db.get(clean.as_str()) {
                let syn = syns[rng.gen_range(0..syns.len())];
                // Preserve original capitalization pattern
                if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                    let mut chars = syn.chars();
                    match chars.next() {
                        Some(first) => {
                            let rest: String = chars.collect();
                            format!("{}{}", first.to_uppercase(), rest)
                        }
                        None => syn.to_string(),
                    }
                } else { syn.to_string() }
            } else { word.to_string() }
        }).collect();

        Ok(result.join(" "))
    }

    fn decode(&self, _input: &str, _params: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Synonym substitution is not reversible".into()))
    }

    fn randomizable(&self) -> bool { false }
}
