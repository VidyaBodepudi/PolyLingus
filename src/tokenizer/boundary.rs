/// Token boundary analysis utilities
/// Provides heuristic detection of likely BPE token boundaries

/// Common BPE merge patterns (simplified)
/// Real tokenizers use learned merges, but these cover common splits
pub fn likely_boundaries(word: &str) -> Vec<usize> {
    let chars: Vec<char> = word.chars().collect();
    let mut boundaries = Vec::new();

    for i in 1..chars.len() {
        let prev = chars[i - 1];
        let curr = chars[i];

        // Boundary heuristics
        let is_boundary =
            // Case change
            (prev.is_lowercase() && curr.is_uppercase()) ||
            // Letter to digit
            (prev.is_alphabetic() && curr.is_numeric()) ||
            // Digit to letter
            (prev.is_numeric() && curr.is_alphabetic()) ||
            // Common suffix boundaries
            (i >= 2 && matches!(&word[i..], s if s.starts_with("ing") || s.starts_with("tion") || s.starts_with("ment") || s.starts_with("ness") || s.starts_with("able") || s.starts_with("ible"))) ||
            // Common prefix boundaries
            (i <= 3 && matches!(&word[..i], s if s == "un" || s == "re" || s == "pre" || s == "dis" || s == "mis"));

        if is_boundary {
            boundaries.push(i);
        }
    }

    boundaries
}
