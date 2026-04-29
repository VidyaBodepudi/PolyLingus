pub mod zero_width;
pub mod scanner;
pub mod whitespace;

#[cfg(feature = "native-image")]
pub mod image_stego;

use crate::core::registry::TransformRegistry;

pub fn register_all(registry: &mut TransformRegistry) {
    registry.register(Box::new(zero_width::ZeroWidthStego));
    registry.register(Box::new(scanner::StegoScanner));
    registry.register(Box::new(whitespace::WhitespaceStego));

    #[cfg(feature = "native-image")]
    registry.register(Box::new(image_stego::ImageStegoTransform));
}
