use std::collections::HashMap;
use crate::core::transform::*;

/// Comprehensive hidden Unicode character scanner
pub struct UnicodeScannerTransform;

impl UnicodeScannerTransform {
    pub fn scan(input: &str) -> UnicodeScanReport {
        let mut findings = Vec::new();
        for (pos, c) in input.char_indices() {
            let cp = c as u32;
            let category = match cp {
                0x200B => Some(("ZWSP", "Zero-Width Space")),
                0x200C => Some(("ZWNJ", "Zero-Width Non-Joiner")),
                0x200D => Some(("ZWJ", "Zero-Width Joiner")),
                0x2060 => Some(("WJ", "Word Joiner")),
                0xFEFF => Some(("BOM", "Byte Order Mark / Zero-Width No-Break Space")),
                0x00AD => Some(("SHY", "Soft Hyphen")),
                0x200E => Some(("LRM", "Left-to-Right Mark")),
                0x200F => Some(("RLM", "Right-to-Left Mark")),
                0x202A => Some(("LRE", "Left-to-Right Embedding")),
                0x202B => Some(("RLE", "Right-to-Left Embedding")),
                0x202C => Some(("PDF", "Pop Directional Formatting")),
                0x202D => Some(("LRO", "Left-to-Right Override")),
                0x202E => Some(("RLO", "Right-to-Left Override")),
                0x2066 => Some(("LRI", "Left-to-Right Isolate")),
                0x2067 => Some(("RLI", "Right-to-Left Isolate")),
                0x2068 => Some(("FSI", "First Strong Isolate")),
                0x2069 => Some(("PDI", "Pop Directional Isolate")),
                0x2028 => Some(("LSEP", "Line Separator")),
                0x2029 => Some(("PSEP", "Paragraph Separator")),
                0xE0000..=0xE007F => Some(("TAG", "Unicode Tag Character")),
                0xFE00..=0xFE0F => Some(("VS", "Variation Selector")),
                0xE0100..=0xE01EF => Some(("SVS", "Supplemental Variation Selector")),
                0x180B..=0x180D => Some(("MVS", "Mongolian Variation Selector")),
                _ if (0x0300..=0x036F).contains(&cp) => Some(("COMB", "Combining Diacritical Mark")),
                _ if (0x1AB0..=0x1AFF).contains(&cp) => Some(("COMB_EXT", "Combining Extended")),
                _ if (0x20D0..=0x20FF).contains(&cp) => Some(("COMB_SYM", "Combining for Symbols")),
                _ if (0xFE20..=0xFE2F).contains(&cp) => Some(("COMB_HALF", "Combining Half Marks")),
                _ => None,
            };
            if let Some((code, desc)) = category {
                findings.push(UnicodeCharFinding { position: pos, codepoint: cp, code: code.to_string(), description: desc.to_string() });
            }
        }
        UnicodeScanReport { total_chars: input.chars().count(), hidden_chars: findings.len(), findings }
    }
}

impl Transform for UnicodeScannerTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "unicode_scan".into(), name: "Unicode Scanner".into(),
            description: "Detect all hidden/invisible Unicode characters".into(),
            category: TransformCategory::Analysis, reversible: false, parameters: vec![],
        }
    }
    fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
        Ok(format!("{}", Self::scan(input)))
    }
    fn decode(&self, _i: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Analysis only".into()))
    }
    fn randomizable(&self) -> bool { false }
}

#[derive(Debug, serde::Serialize)]
pub struct UnicodeScanReport { pub total_chars: usize, pub hidden_chars: usize, pub findings: Vec<UnicodeCharFinding> }

#[derive(Debug, serde::Serialize)]
pub struct UnicodeCharFinding { pub position: usize, pub codepoint: u32, pub code: String, pub description: String }

impl std::fmt::Display for UnicodeScanReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Unicode Scanner ===")?;
        writeln!(f, "Total: {} chars | Hidden: {}", self.total_chars, self.hidden_chars)?;
        if self.findings.is_empty() { writeln!(f, "✓ No hidden Unicode characters")?; }
        else {
            for finding in &self.findings {
                writeln!(f, "  pos {}: U+{:04X} [{}] {}", finding.position, finding.codepoint, finding.code, finding.description)?;
            }
        }
        Ok(())
    }
}
