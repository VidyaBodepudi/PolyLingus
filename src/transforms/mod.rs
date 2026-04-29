pub mod encodings;
pub mod ciphers;
pub mod visual;
pub mod unicode_styles;
pub mod scripts;
pub mod formatting;
pub mod advanced_ciphers;

use crate::core::registry::TransformRegistry;

/// Register all transform implementations
pub fn register_all(registry: &mut TransformRegistry) {
    encodings::register(registry);
    ciphers::register(registry);
    visual::register(registry);
    unicode_styles::register(registry);
    scripts::register(registry);
    formatting::register(registry);
    advanced_ciphers::register(registry);
}
