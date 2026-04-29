pub mod confusables;
pub mod generator;
pub mod detector;

use crate::core::registry::TransformRegistry;

pub fn register_all(registry: &mut TransformRegistry) {
    registry.register(Box::new(generator::HomoglyphGenerator));
    registry.register(Box::new(detector::HomoglyphDetector));
}
