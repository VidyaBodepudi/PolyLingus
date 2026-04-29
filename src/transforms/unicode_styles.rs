use std::collections::HashMap;
use crate::core::registry::TransformRegistry;
use crate::core::transform::*;

fn style_map(input: &str, upper_start: u32, lower_start: u32) -> String {
    input.chars().map(|c| {
        if c.is_ascii_uppercase() { char::from_u32(upper_start + (c as u32 - 'A' as u32)).unwrap_or(c) }
        else if c.is_ascii_lowercase() { char::from_u32(lower_start + (c as u32 - 'a' as u32)).unwrap_or(c) }
        else { c }
    }).collect()
}

fn reverse_style(input: &str, upper_start: u32, lower_start: u32) -> String {
    input.chars().map(|c| {
        let cp = c as u32;
        if cp >= upper_start && cp < upper_start + 26 { (b'A' + (cp - upper_start) as u8) as char }
        else if cp >= lower_start && cp < lower_start + 26 { (b'a' + (cp - lower_start) as u8) as char }
        else { c }
    }).collect()
}

macro_rules! unicode_style {
    ($name:ident, $key:expr, $display:expr, $desc:expr, $upper:expr, $lower:expr) => {
        pub struct $name;
        impl Transform for $name {
            fn info(&self) -> TransformInfo {
                TransformInfo {
                    key: $key.into(), name: $display.into(), description: $desc.into(),
                    category: TransformCategory::UnicodeStyle, reversible: true, parameters: vec![],
                }
            }
            fn encode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
                Ok(style_map(input, $upper, $lower))
            }
            fn decode(&self, input: &str, _p: &HashMap<String, String>) -> TransformResult {
                Ok(reverse_style(input, $upper, $lower))
            }
        }
    };
}

unicode_style!(FrakturStyle, "fraktur", "Fraktur", "Mathematical Fraktur alphabet", 0x1D504, 0x1D51E);
unicode_style!(CursiveStyle, "cursive", "Cursive", "Cursive/script style", 0x1D49C, 0x1D4B6);
unicode_style!(MonospaceStyle, "monospace", "Monospace", "Monospace math characters", 0x1D670, 0x1D68A);
unicode_style!(DoubleStruckStyle, "double_struck", "Double-Struck", "Mathematical double-struck", 0x1D538, 0x1D552);
unicode_style!(BoldStyle, "bold", "Bold", "Mathematical bold letters", 0x1D400, 0x1D41A);
unicode_style!(ItalicStyle, "italic", "Italic", "Mathematical italic letters", 0x1D434, 0x1D44E);
unicode_style!(BoldItalicStyle, "bold_italic", "Bold Italic", "Mathematical bold italic", 0x1D468, 0x1D482);
unicode_style!(SansSerifStyle, "sans_serif", "Sans-Serif", "Mathematical sans-serif", 0x1D5A0, 0x1D5BA);
unicode_style!(SansSerifBoldStyle, "sans_bold", "Sans-Serif Bold", "Math sans-serif bold", 0x1D5D4, 0x1D5EE);

pub fn register(registry: &mut TransformRegistry) {
    registry.register(Box::new(FrakturStyle));
    registry.register(Box::new(CursiveStyle));
    registry.register(Box::new(MonospaceStyle));
    registry.register(Box::new(DoubleStruckStyle));
    registry.register(Box::new(BoldStyle));
    registry.register(Box::new(ItalicStyle));
    registry.register(Box::new(BoldItalicStyle));
    registry.register(Box::new(SansSerifStyle));
    registry.register(Box::new(SansSerifBoldStyle));
}
