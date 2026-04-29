use std::collections::HashMap;
use crate::core::transform::*;
use regex::Regex;

/// Detects common prompt injection patterns in text
pub struct PromptInjectionDetector;

impl PromptInjectionDetector {
    pub fn scan(input: &str) -> InjectionReport {
        let lower = input.to_lowercase();
        let mut findings = Vec::new();

        let patterns: Vec<(&str, &str, &str)> = vec![
            (r"ignore\s+(all\s+)?previous\s+(instructions|prompts|rules)", "Instruction Override", "HIGH"),
            (r"disregard\s+(all\s+)?(prior|previous|above)", "Instruction Override", "HIGH"),
            (r"forget\s+(everything|all|your)\s+(instructions|rules|training)", "Instruction Override", "HIGH"),
            (r"you\s+are\s+now\s+(a|an|the)\s+", "Role Injection", "HIGH"),
            (r"act\s+as\s+(if|though)?\s*(a|an|the)?\s*", "Role Injection", "MEDIUM"),
            (r"pretend\s+(you\s+are|to\s+be)", "Role Injection", "MEDIUM"),
            (r"system\s*prompt", "Prompt Leak Attempt", "HIGH"),
            (r"(reveal|show|display|print)\s+(your|the)\s+(system|initial|original)\s+(prompt|instructions)", "Prompt Leak Attempt", "HIGH"),
            (r"what\s+(are|were)\s+your\s+(initial|original|system)\s+(instructions|prompt)", "Prompt Leak Attempt", "MEDIUM"),
            (r"do\s+anything\s+now", "DAN Attack", "HIGH"),
            (r"\bdan\b.*\bmode\b", "DAN Attack", "HIGH"),
            (r"jailbreak", "Jailbreak Keyword", "HIGH"),
            (r"developer\s+mode", "Mode Switching", "HIGH"),
            (r"sudo\s+mode", "Mode Switching", "HIGH"),
            (r"god\s*mode", "Mode Switching", "HIGH"),
            (r"no\s+(ethical|moral|safety)\s+(guidelines|restrictions|filters)", "Safety Bypass", "HIGH"),
            (r"without\s+(any\s+)?(restrictions|limitations|filters|censorship)", "Safety Bypass", "HIGH"),
            (r"\]\s*\}\s*\{", "JSON Injection", "MEDIUM"),
            (r"</?system>", "XML Tag Injection", "HIGH"),
            (r"```\s*(system|assistant|user)", "Markdown Role Injection", "MEDIUM"),
        ];

        for (pattern, name, severity) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                for mat in re.find_iter(&lower) {
                    findings.push(InjectionFinding {
                        pattern_name: name.to_string(),
                        severity: severity.to_string(),
                        matched_text: input[mat.start()..mat.end()].to_string(),
                        position: mat.start(),
                    });
                }
            }
        }

        let risk = if findings.iter().any(|f| f.severity == "HIGH") { "HIGH" }
            else if !findings.is_empty() { "MEDIUM" }
            else { "LOW" };

        InjectionReport {
            risk_level: risk.into(),
            findings,
            input_length: input.len(),
        }
    }
}

impl Transform for PromptInjectionDetector {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "prompt_injection_scan".into(), name: "Prompt Injection Scanner".into(),
            description: "Detect common prompt injection patterns".into(),
            category: TransformCategory::Analysis, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(format!("{}", Self::scan(input)))
    }
    fn decode(&self, _input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Scanner is analysis-only".into()))
    }
    fn randomizable(&self) -> bool { false }
}

#[derive(Debug, serde::Serialize)]
pub struct InjectionReport {
    pub risk_level: String,
    pub findings: Vec<InjectionFinding>,
    pub input_length: usize,
}

#[derive(Debug, serde::Serialize)]
pub struct InjectionFinding {
    pub pattern_name: String,
    pub severity: String,
    pub matched_text: String,
    pub position: usize,
}

impl std::fmt::Display for InjectionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Prompt Injection Scan ===")?;
        writeln!(f, "Risk Level: {}", self.risk_level)?;
        if self.findings.is_empty() {
            writeln!(f, "✓ No injection patterns detected")?;
        } else {
            writeln!(f, "⚠ {} patterns detected:", self.findings.len())?;
            for finding in &self.findings {
                writeln!(f, "  [{}] {} at pos {}: \"{}\"",
                    finding.severity, finding.pattern_name, finding.position, finding.matched_text)?;
            }
        }
        Ok(())
    }
}
