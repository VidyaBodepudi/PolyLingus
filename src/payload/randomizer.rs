use std::collections::HashMap;
use rand::Rng;
use crate::core::registry::TransformRegistry;

/// Applies a random different transform to each word
pub struct Randomizer;

impl Randomizer {
    pub fn randomize(input: &str, registry: &TransformRegistry) -> String {
        let transforms = registry.randomizable();
        if transforms.is_empty() { return input.to_string(); }

        let mut rng = rand::thread_rng();
        let empty_params = HashMap::new();

        input.split_whitespace().map(|word| {
            let idx = rng.gen_range(0..transforms.len());
            transforms[idx].encode(word, &empty_params).unwrap_or_else(|_| word.to_string())
        }).collect::<Vec<_>>().join(" ")
    }
}
