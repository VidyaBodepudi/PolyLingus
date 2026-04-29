pub mod synonyms;
pub mod paraphrase;
pub mod euphemisms;
pub mod register;
pub mod fragmentation;

use crate::core::registry::TransformRegistry;

pub fn register_all(registry: &mut TransformRegistry) {
    registry.register(Box::new(synonyms::SynonymTransform));
    registry.register(Box::new(paraphrase::ParaphraseTransform));
    registry.register(Box::new(euphemisms::EuphemismTransform));
    registry.register(Box::new(register::RegisterShift));
    registry.register(Box::new(fragmentation::SemanticFragmenter));
}
