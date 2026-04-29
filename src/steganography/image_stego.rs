use std::collections::HashMap;
use crate::core::transform::*;

/// Image steganography using LSB (Least Significant Bit) technique
pub struct ImageStegoTransform;

impl ImageStegoTransform {
    /// Embed a message into a PNG image using LSB encoding
    pub fn embed(cover_path: &str, message: &str, output_path: &str) -> Result<(), TransformError> {
        let img = image::open(cover_path).map_err(|e| TransformError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        let mut img = img.to_rgba8();
        let (w, h) = img.dimensions();
        let capacity = (w * h * 3 / 8) as usize; // 3 channels, 1 bit each, 8 bits per byte

        let msg_bytes = message.as_bytes();
        // Prepend length as 4-byte big-endian
        let len = msg_bytes.len() as u32;
        let mut payload = Vec::with_capacity(4 + msg_bytes.len());
        payload.extend_from_slice(&len.to_be_bytes());
        payload.extend_from_slice(msg_bytes);

        if payload.len() > capacity {
            return Err(TransformError::EncodingError(
                format!("Message too large: {} bytes, capacity: {} bytes", payload.len(), capacity)));
        }

        let mut bit_idx = 0usize;
        'outer: for y in 0..h {
            for x in 0..w {
                let pixel = img.get_pixel_mut(x, y);
                for channel in 0..3u8 { // R, G, B only
                    if bit_idx >= payload.len() * 8 { break 'outer; }
                    let byte_idx = bit_idx / 8;
                    let bit_pos = 7 - (bit_idx % 8);
                    let bit = (payload[byte_idx] >> bit_pos) & 1;
                    pixel[channel as usize] = (pixel[channel as usize] & 0xFE) | bit;
                    bit_idx += 1;
                }
            }
        }

        img.save(output_path).map_err(|e| TransformError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        Ok(())
    }

    /// Extract a hidden message from a PNG image
    pub fn extract(image_path: &str) -> Result<String, TransformError> {
        let img = image::open(image_path).map_err(|e| TransformError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();

        // First extract 32 bits for the length
        let mut bits = Vec::new();
        'outer: for y in 0..h {
            for x in 0..w {
                let pixel = img.get_pixel(x, y);
                for channel in 0..3u8 {
                    bits.push(pixel[channel as usize] & 1);
                    if bits.len() >= 32 + 8 * 65536 { break 'outer; } // safety limit
                }
            }
        }

        if bits.len() < 32 {
            return Err(TransformError::DecodingError("Image too small".into()));
        }

        let mut len: u32 = 0;
        for i in 0..32 { len = (len << 1) | bits[i] as u32; }

        if len as usize > bits.len() / 8 - 4 {
            return Err(TransformError::DecodingError("Invalid message length".into()));
        }

        let mut bytes = Vec::with_capacity(len as usize);
        for i in 0..len as usize {
            let mut byte: u8 = 0;
            for bit in 0..8 {
                byte = (byte << 1) | bits[32 + i * 8 + bit];
            }
            bytes.push(byte);
        }

        String::from_utf8(bytes).map_err(|e| TransformError::DecodingError(e.to_string()))
    }
}

impl Transform for ImageStegoTransform {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "image_stego".into(), name: "Image Steganography".into(),
            description: "Hide messages in PNG images using LSB technique".into(),
            category: TransformCategory::Steganography, reversible: true,
            parameters: vec![
                ParameterInfo { name: "cover".into(), description: "Path to cover image".into(),
                    default_value: "".into(), param_type: ParamType::Text },
                ParameterInfo { name: "output".into(), description: "Output path".into(),
                    default_value: "stego_output.png".into(), param_type: ParamType::Text },
                ParameterInfo { name: "image".into(), description: "Path to stego image (for extraction)".into(),
                    default_value: "".into(), param_type: ParamType::Text },
            ],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let cover = params.get("cover").ok_or_else(|| TransformError::InvalidParameter("cover image path required".into()))?;
        let output = params.get("output").map(|s| s.as_str()).unwrap_or("stego_output.png");
        Self::embed(cover, input, output)?;
        Ok(format!("Message embedded in {}", output))
    }

    fn decode(&self, _input: &str, params: &HashMap<String, String>) -> TransformResult {
        let image = params.get("image").ok_or_else(|| TransformError::InvalidParameter("image path required".into()))?;
        Self::extract(image)
    }

    fn randomizable(&self) -> bool { false }
}
