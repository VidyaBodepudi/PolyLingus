use std::collections::HashMap;
use super::transform::{Transform, TransformCategory, TransformInfo};

/// Central registry that holds all available transforms
pub struct TransformRegistry {
    transforms: HashMap<String, Box<dyn Transform>>,
}

impl TransformRegistry {
    pub fn new() -> Self {
        Self {
            transforms: HashMap::new(),
        }
    }

    /// Register a new transform
    pub fn register(&mut self, transform: Box<dyn Transform>) {
        let key = transform.info().key.clone();
        self.transforms.insert(key, transform);
    }

    /// Get a transform by key
    pub fn get(&self, key: &str) -> Option<&dyn Transform> {
        self.transforms.get(key).map(|t| t.as_ref())
    }

    /// List all registered transforms
    pub fn list_all(&self) -> Vec<TransformInfo> {
        let mut infos: Vec<_> = self.transforms.values().map(|t| t.info()).collect();
        infos.sort_by(|a, b| a.name.cmp(&b.name));
        infos
    }

    /// List transforms by category
    pub fn list_by_category(&self, category: &TransformCategory) -> Vec<TransformInfo> {
        self.transforms
            .values()
            .filter(|t| &t.info().category == category)
            .map(|t| t.info())
            .collect()
    }

    /// Get all transform keys
    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<_> = self.transforms.keys().cloned().collect();
        keys.sort();
        keys
    }

    /// Get all transforms that support randomization
    pub fn randomizable(&self) -> Vec<&dyn Transform> {
        self.transforms
            .values()
            .filter(|t| t.randomizable())
            .map(|t| t.as_ref())
            .collect()
    }

    /// Get the total number of registered transforms
    pub fn count(&self) -> usize {
        self.transforms.len()
    }

    /// Build a registry with all default transforms
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        crate::transforms::register_all(&mut registry);
        crate::homoglyph::register_all(&mut registry);
        crate::steganography::register_all(&mut registry);
        crate::analysis::register_all(&mut registry);
        crate::semantic::register_all(&mut registry);
        crate::tokenizer::register_all(&mut registry);
        registry
    }
}

impl Default for TransformRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
