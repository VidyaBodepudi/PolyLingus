use std::collections::HashMap;
use super::registry::TransformRegistry;
use super::transform::{TransformError, TransformResult};

/// A pipeline is a named sequence of transforms applied in order
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<PipelineStep>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineStep {
    pub transform_key: String,
    pub params: HashMap<String, String>,
}

impl Pipeline {
    /// Create a new empty pipeline
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: Vec::new(),
        }
    }

    /// Add a step to the pipeline
    pub fn add_step(&mut self, transform_key: &str, params: HashMap<String, String>) {
        self.steps.push(PipelineStep {
            transform_key: transform_key.to_string(),
            params,
        });
    }

    /// Parse a pipeline from a string like "rot13 -> base64 -> hex"
    pub fn from_chain_str(name: &str, chain: &str) -> Self {
        let steps: Vec<PipelineStep> = chain
            .split("->")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| PipelineStep {
                transform_key: s.to_lowercase().replace(' ', "_"),
                params: HashMap::new(),
            })
            .collect();

        Self {
            name: name.to_string(),
            steps,
        }
    }

    /// Execute the pipeline in the forward (encode) direction
    pub fn encode(&self, input: &str, registry: &TransformRegistry) -> TransformResult {
        let mut current = input.to_string();
        for step in &self.steps {
            let transform = registry.get(&step.transform_key).ok_or_else(|| {
                TransformError::EncodingError(format!(
                    "Unknown transform in pipeline: '{}'",
                    step.transform_key
                ))
            })?;
            current = transform.encode(&current, &step.params)?;
        }
        Ok(current)
    }

    /// Execute the pipeline in reverse (decode) direction
    pub fn decode(&self, input: &str, registry: &TransformRegistry) -> TransformResult {
        let mut current = input.to_string();
        // Reverse the steps for decoding
        for step in self.steps.iter().rev() {
            let transform = registry.get(&step.transform_key).ok_or_else(|| {
                TransformError::DecodingError(format!(
                    "Unknown transform in pipeline: '{}'",
                    step.transform_key
                ))
            })?;
            current = transform.decode(&current, &step.params)?;
        }
        Ok(current)
    }

    /// Generate the reverse pipeline
    pub fn reversed(&self) -> Self {
        let mut reversed = self.clone();
        reversed.name = format!("{}_reversed", self.name);
        reversed.steps.reverse();
        reversed
    }

    /// Serialize to JSON for saving
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
