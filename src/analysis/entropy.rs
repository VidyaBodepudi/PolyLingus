use std::collections::HashMap;
use crate::core::transform::*;

/// Analyze text entropy to detect encoded/encrypted payloads
pub struct EntropyAnalyzer;

impl EntropyAnalyzer {
    /// Calculate Shannon entropy of a string
    pub fn shannon_entropy(input: &str) -> f64 {
        if input.is_empty() { return 0.0; }
        let mut freq: HashMap<char, usize> = HashMap::new();
        let len = input.chars().count() as f64;
        for c in input.chars() { *freq.entry(c).or_default() += 1; }
        freq.values().fold(0.0, |acc, &count| {
            let p = count as f64 / len;
            acc - p * p.log2()
        })
    }

    /// Analyze entropy per chunk to find anomalous regions
    pub fn analyze(input: &str, chunk_size: usize) -> EntropyReport {
        let overall = Self::shannon_entropy(input);
        let chars: Vec<char> = input.chars().collect();
        let chunks: Vec<ChunkEntropy> = chars.chunks(chunk_size.max(1)).enumerate().map(|(i, chunk)| {
            let text: String = chunk.iter().collect();
            let entropy = Self::shannon_entropy(&text);
            ChunkEntropy { offset: i * chunk_size, length: chunk.len(), entropy, text_preview: text.chars().take(30).collect() }
        }).collect();

        let avg = if chunks.is_empty() { 0.0 } else {
            chunks.iter().map(|c| c.entropy).sum::<f64>() / chunks.len() as f64
        };
        let anomalous: Vec<usize> = chunks.iter().enumerate()
            .filter(|(_, c)| (c.entropy - avg).abs() > 1.5)
            .map(|(i, _)| i).collect();

        let assessment = if overall > 4.5 { "HIGH — likely encoded/encrypted content" }
            else if overall > 3.5 { "MODERATE — mixed content, possible embedded payloads" }
            else if overall > 2.0 { "NORMAL — natural language text" }
            else { "LOW — highly repetitive or simple content" };

        EntropyReport { overall_entropy: overall, average_chunk_entropy: avg, chunks, anomalous_chunks: anomalous, assessment: assessment.into() }
    }
}

impl Transform for EntropyAnalyzer {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "entropy".into(), name: "Entropy Analysis".into(),
            description: "Analyze text entropy to detect encoded/encrypted payloads".into(),
            category: TransformCategory::Analysis, reversible: false, parameters: vec![
                ParameterInfo { name: "chunk".into(), description: "Chunk size for analysis".into(),
                    default_value: "64".into(), param_type: ParamType::Integer { min: 8, max: 1024 } },
            ],
        }
    }
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let chunk = params.get("chunk").and_then(|s| s.parse().ok()).unwrap_or(64);
        Ok(format!("{}", Self::analyze(input, chunk)))
    }
    fn decode(&self, _i: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Analysis only".into()))
    }
    fn randomizable(&self) -> bool { false }
}

#[derive(Debug, serde::Serialize)]
pub struct EntropyReport { pub overall_entropy: f64, pub average_chunk_entropy: f64, pub chunks: Vec<ChunkEntropy>, pub anomalous_chunks: Vec<usize>, pub assessment: String }

#[derive(Debug, serde::Serialize)]
pub struct ChunkEntropy { pub offset: usize, pub length: usize, pub entropy: f64, pub text_preview: String }

impl std::fmt::Display for EntropyReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Entropy Analysis ===")?;
        writeln!(f, "Overall entropy: {:.3} bits/char", self.overall_entropy)?;
        writeln!(f, "Assessment: {}", self.assessment)?;
        if !self.anomalous_chunks.is_empty() {
            writeln!(f, "⚠ Anomalous chunks: {:?}", self.anomalous_chunks)?;
        }
        Ok(())
    }
}
