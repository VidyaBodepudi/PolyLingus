use std::collections::HashMap;
use crate::core::transform::*;

/// Scans text for hidden steganographic content
pub struct StegoScanner;

impl StegoScanner {
    pub fn scan(input: &str) -> StegoReport {
        let mut findings = Vec::new();

        // Check for zero-width characters
        let zw_count = input.chars().filter(|c| matches!(*c as u32,
            0x200B | 0x200C | 0x200D | 0x2060 | 0xFEFF
        )).count();
        if zw_count > 0 {
            findings.push(StegoFinding {
                finding_type: "Zero-Width Characters".into(),
                severity: "HIGH".into(),
                description: format!("Found {} zero-width characters that may encode hidden data", zw_count),
                count: zw_count,
            });
        }

        // Check for Unicode Tags (U+E0000-E007F)
        let tag_count = input.chars().filter(|c| (0xE0000..=0xE007F).contains(&(*c as u32))).count();
        if tag_count > 0 {
            findings.push(StegoFinding {
                finding_type: "Unicode Tags".into(), severity: "HIGH".into(),
                description: format!("Found {} Unicode tag characters (invisible metadata plane)", tag_count),
                count: tag_count,
            });
        }

        // Check for variation selectors
        let vs_count = input.chars().filter(|c| (0xFE00..=0xFE0F).contains(&(*c as u32))
            || (0xE0100..=0xE01EF).contains(&(*c as u32))).count();
        if vs_count > 0 {
            findings.push(StegoFinding {
                finding_type: "Variation Selectors".into(), severity: "MEDIUM".into(),
                description: format!("Found {} variation selectors (may be emoji steganography)", vs_count),
                count: vs_count,
            });
        }

        // Check for RTL/LTR override characters
        let bidi_count = input.chars().filter(|c| matches!(*c as u32,
            0x200E | 0x200F | 0x202A | 0x202B | 0x202C | 0x202D | 0x202E |
            0x2066 | 0x2067 | 0x2068 | 0x2069
        )).count();
        if bidi_count > 0 {
            findings.push(StegoFinding {
                finding_type: "Bidi Override".into(), severity: "HIGH".into(),
                description: format!("Found {} bidirectional override chars (text direction manipulation)", bidi_count),
                count: bidi_count,
            });
        }

        // Check for excessive combining marks (potential Zalgo)
        let combining_count = input.chars().filter(|c| {
            let cp = *c as u32;
            (0x0300..=0x036F).contains(&cp) || (0x0483..=0x0489).contains(&cp) ||
            (0x1DC0..=0x1DFF).contains(&cp) || (0x20D0..=0x20FF).contains(&cp) ||
            (0xFE20..=0xFE2F).contains(&cp)
        }).count();
        if combining_count > input.chars().count() / 3 {
            findings.push(StegoFinding {
                finding_type: "Combining Mark Abuse".into(), severity: "MEDIUM".into(),
                description: format!("Excessive combining marks ({}) — possible Zalgo or data encoding", combining_count),
                count: combining_count,
            });
        }

        StegoReport {
            total_chars: input.chars().count(),
            visible_chars: input.chars().filter(|c| !c.is_control() && c.is_alphanumeric() || c.is_whitespace()).count(),
            findings,
        }
    }
}

impl Transform for StegoScanner {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "stego_scan".into(), name: "Steganography Scanner".into(),
            description: "Scan for hidden content in text".into(),
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
pub struct StegoReport {
    pub total_chars: usize,
    pub visible_chars: usize,
    pub findings: Vec<StegoFinding>,
}

#[derive(Debug, serde::Serialize)]
pub struct StegoFinding {
    pub finding_type: String,
    pub severity: String,
    pub description: String,
    pub count: usize,
}

impl std::fmt::Display for StegoReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Steganography Scan ===")?;
        writeln!(f, "Total characters: {} | Visible: {}", self.total_chars, self.visible_chars)?;
        if self.findings.is_empty() {
            writeln!(f, "✓ No hidden content detected")?;
        } else {
            writeln!(f, "⚠ {} findings:", self.findings.len())?;
            for finding in &self.findings {
                writeln!(f, "  [{}] {}: {}", finding.severity, finding.finding_type, finding.description)?;
            }
        }
        Ok(())
    }
}
