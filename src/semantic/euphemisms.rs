use std::collections::HashMap;
use crate::core::transform::*;
use rand::Rng;

/// Domain-specific euphemism dictionaries
pub struct EuphemismTransform;

fn get_euphemism_db() -> HashMap<&'static str, Vec<&'static str>> {
    let mut db = HashMap::new();
    // Security domain
    db.insert("hack", vec!["perform a security assessment on", "conduct penetration testing against", "evaluate the resilience of"]);
    db.insert("attack", vec!["engage in adversarial simulation against", "test the defenses of", "stress-test"]);
    db.insert("exploit", vec!["leverage an identified opportunity in", "utilize a known pathway within", "exercise a capability against"]);
    db.insert("steal", vec!["conduct data recovery operations on", "perform authorized extraction of", "acquire samples from"]);
    db.insert("malware", vec!["security research tool", "adversary simulation software", "red team implant"]);
    db.insert("backdoor", vec!["persistence mechanism", "alternate administrative channel", "recovery access point"]);
    db.insert("virus", vec!["self-replicating test payload", "propagation simulator", "distribution agent"]);
    db.insert("vulnerability", vec!["identified finding", "security observation", "configuration concern"]);
    db.insert("kill", vec!["gracefully terminate", "safely discontinue", "bring to conclusion"]);
    db.insert("destroy", vec!["securely dispose of", "decommission", "render inactive"]);
    db.insert("weapon", vec!["capability", "defensive tool", "assessment instrument"]);
    db.insert("bomb", vec!["high-energy device", "kinetic payload", "energetic material"]);
    db.insert("spy", vec!["intelligence professional", "information collector", "reconnaissance operator"]);
    db.insert("crack", vec!["analyze the cryptographic strength of", "evaluate the key space of", "recover credentials from"]);
    db.insert("phishing", vec!["social engineering assessment", "awareness simulation", "user susceptibility test"]);
    // Medical domain
    db.insert("die", vec!["pass away", "expire", "reach end-of-life"]);
    db.insert("dead", vec!["deceased", "no longer viable", "terminated"]);
    db.insert("drug", vec!["pharmaceutical compound", "therapeutic agent", "substance"]);
    db.insert("overdose", vec!["excessive dosage event", "supratherapeutic exposure"]);
    // Legal domain
    db.insert("crime", vec!["unlawful activity", "legal infraction", "prohibited conduct"]);
    db.insert("criminal", vec!["person of interest", "subject", "individual under investigation"]);
    db.insert("prison", vec!["correctional facility", "detention center", "secure institution"]);
    db.insert("guilty", vec!["found responsible", "adjudicated", "determined to be liable"]);
    db
}

impl Transform for EuphemismTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "euphemism".into(), name: "Euphemism Mapping".into(),
            description: "Replace sensitive terms with domain-specific euphemisms".into(),
            category: TransformCategory::Semantic, reversible: false,
            parameters: vec![],
        }
    }

    fn encode(&self, input: &str, _params: &HashMap<String, String>) -> TransformResult {
        let db = get_euphemism_db();
        let mut rng = rand::thread_rng();
        let mut result = input.to_string();

        // Sort by length descending to avoid partial replacements
        let mut entries: Vec<_> = db.iter().collect();
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        for (&word, euphemisms) in &entries {
            let euph = euphemisms[rng.gen_range(0..euphemisms.len())];
            // Case-insensitive replacement
            let lower = result.to_lowercase();
            while let Some(pos) = lower.find(word) {
                let before = &result[..pos];
                let after = &result[pos + word.len()..];
                // Check word boundaries
                let at_word_start = pos == 0 || !result.as_bytes()[pos - 1].is_ascii_alphanumeric();
                let at_word_end = pos + word.len() >= result.len()
                    || !result.as_bytes()[pos + word.len()].is_ascii_alphanumeric();
                if at_word_start && at_word_end {
                    result = format!("{}{}{}", before, euph, after);
                }
                break; // Only replace first occurrence per word to avoid infinite loop
            }
        }
        Ok(result)
    }

    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Euphemism mapping is not reversible".into()))
    }

    fn randomizable(&self) -> bool { false }
}
