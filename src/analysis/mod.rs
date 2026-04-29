pub mod prompt_injection;
pub mod entropy;
pub mod unicode_scanner;
pub mod report;

use crate::core::registry::TransformRegistry;

pub fn register_all(registry: &mut TransformRegistry) {
    registry.register(Box::new(prompt_injection::PromptInjectionDetector));
    registry.register(Box::new(entropy::EntropyAnalyzer));
    registry.register(Box::new(unicode_scanner::UnicodeScannerTransform));
}
