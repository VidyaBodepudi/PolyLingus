use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during transformation
#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Transform failed: {0}")]
    EncodingError(String),

    #[error("Decoding failed: {0}")]
    DecodingError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type TransformResult = Result<String, TransformError>;

/// Category of a transform for UI organization and filtering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransformCategory {
    Encoding,
    Cipher,
    Visual,
    UnicodeStyle,
    Script,
    Formatting,
    Steganography,
    Semantic,
    Homoglyph,
    Analysis,
}

impl std::fmt::Display for TransformCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding => write!(f, "Encoding"),
            Self::Cipher => write!(f, "Cipher"),
            Self::Visual => write!(f, "Visual"),
            Self::UnicodeStyle => write!(f, "Unicode Style"),
            Self::Script => write!(f, "Script"),
            Self::Formatting => write!(f, "Formatting"),
            Self::Steganography => write!(f, "Steganography"),
            Self::Semantic => write!(f, "Semantic"),
            Self::Homoglyph => write!(f, "Homoglyph"),
            Self::Analysis => write!(f, "Analysis"),
        }
    }
}

/// Metadata about a transform
#[derive(Debug, Clone)]
pub struct TransformInfo {
    /// Machine-readable key (e.g., "base64", "caesar")
    pub key: String,
    /// Human-readable name (e.g., "Base64 Encoding")
    pub name: String,
    /// Brief description
    pub description: String,
    /// Category for organization
    pub category: TransformCategory,
    /// Whether this transform supports decoding/reversal
    pub reversible: bool,
    /// Optional parameters this transform accepts
    pub parameters: Vec<ParameterInfo>,
}

/// Describes a configurable parameter for a transform
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub description: String,
    pub default_value: String,
    pub param_type: ParamType,
}

#[derive(Debug, Clone)]
pub enum ParamType {
    Integer { min: i64, max: i64 },
    Text,
    Choice(Vec<String>),
    Boolean,
}

/// The core trait that all transforms must implement
pub trait Transform: Send + Sync {
    /// Get metadata about this transform
    fn info(&self) -> TransformInfo;

    /// Encode/transform the input text
    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult;

    /// Decode/reverse the transformed text (if supported)
    fn decode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult;

    /// Generate a short preview of what this transform does
    fn preview(&self, input: &str) -> String {
        let sample = if input.len() > 20 { &input[..20] } else { input };
        match self.encode(sample, &HashMap::new()) {
            Ok(result) => {
                if result.len() > 40 {
                    format!("{}...", &result[..40])
                } else {
                    result
                }
            }
            Err(_) => String::from("[preview unavailable]"),
        }
    }

    /// Whether this transform can be used in the randomizer
    fn randomizable(&self) -> bool {
        true
    }
}
