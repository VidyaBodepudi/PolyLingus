use crate::analysis::{prompt_injection, entropy, unicode_scanner};
use crate::steganography::scanner::StegoScanner;
use crate::homoglyph::detector::HomoglyphDetector;

/// Generate a comprehensive analysis report in JSON or text
pub struct ReportGenerator;

impl ReportGenerator {
    pub fn full_analysis(input: &str) -> FullReport {
        FullReport {
            prompt_injection: prompt_injection::PromptInjectionDetector::scan(input),
            entropy: entropy::EntropyAnalyzer::analyze(input, 64),
            unicode: unicode_scanner::UnicodeScannerTransform::scan(input),
            steganography: StegoScanner::scan(input),
            homoglyph: HomoglyphDetector::analyze(input),
        }
    }

    pub fn to_json(input: &str) -> Result<String, serde_json::Error> {
        let report = Self::full_analysis(input);
        serde_json::to_string_pretty(&report)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct FullReport {
    pub prompt_injection: prompt_injection::InjectionReport,
    pub entropy: entropy::EntropyReport,
    pub unicode: unicode_scanner::UnicodeScanReport,
    pub steganography: crate::steganography::scanner::StegoReport,
    pub homoglyph: crate::homoglyph::detector::HomoglyphReport,
}

impl std::fmt::Display for FullReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.prompt_injection)?;
        writeln!(f, "{}", self.entropy)?;
        writeln!(f, "{}", self.unicode)?;
        writeln!(f, "{}", self.steganography)?;
        writeln!(f, "{}", self.homoglyph)?;
        Ok(())
    }
}
