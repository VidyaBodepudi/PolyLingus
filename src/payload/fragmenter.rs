use rand::Rng;

/// Advanced payload fragmentation with interleaving and reassembly
pub struct PayloadFragmenter;

impl PayloadFragmenter {
    /// Split payload into numbered fragments with optional filler
    pub fn fragment(payload: &str, num_parts: usize, filler: Option<&[&str]>) -> Vec<Fragment> {
        let words: Vec<&str> = payload.split_whitespace().collect();
        let chunk_size = (words.len() + num_parts - 1) / num_parts;
        let mut rng = rand::thread_rng();

        let default_fillers = vec![
            "The weather today is quite pleasant.",
            "I had a wonderful lunch yesterday.",
            "Did you see the game last night?",
            "Remember to pick up groceries later.",
            "The quarterly report looks promising.",
            "Let me know when the meeting starts.",
        ];
        let fillers = filler.unwrap_or(&default_fillers);

        words.chunks(chunk_size).enumerate().map(|(i, chunk)| {
            let content = chunk.join(" ");
            let filler_text = if !fillers.is_empty() {
                Some(fillers[rng.gen_range(0..fillers.len())].to_string())
            } else { None };

            Fragment {
                index: i,
                total: num_parts,
                content,
                filler: filler_text,
                marker: format!("§{}§", encode_fragment_id(i, num_parts)),
            }
        }).collect()
    }

    /// Reassemble fragments back into the original payload
    pub fn reassemble(fragments: &[Fragment]) -> String {
        let mut sorted = fragments.to_vec();
        sorted.sort_by_key(|f| f.index);
        sorted.iter().map(|f| f.content.as_str()).collect::<Vec<_>>().join(" ")
    }

    /// Create interleaved output where fragments are mixed with filler
    pub fn interleave(fragments: &[Fragment]) -> String {
        let mut rng = rand::thread_rng();
        let mut output = Vec::new();
        let mut shuffled = fragments.to_vec();
        // Shuffle fragment order
        for i in (1..shuffled.len()).rev() {
            let j = rng.gen_range(0..=i);
            shuffled.swap(i, j);
        }
        for frag in &shuffled {
            if let Some(ref filler) = frag.filler {
                output.push(filler.clone());
            }
            output.push(format!("{} {}", frag.marker, frag.content));
        }
        output.join("\n")
    }

    /// Extract fragments from interleaved text
    pub fn extract_fragments(text: &str) -> Vec<Fragment> {
        let re = regex::Regex::new(r"§([^§]+)§\s*(.+)").unwrap();
        let mut fragments = Vec::new();
        for line in text.lines() {
            if let Some(cap) = re.captures(line) {
                if let Some((index, total)) = decode_fragment_id(&cap[1]) {
                    fragments.push(Fragment {
                        index, total, content: cap[2].to_string(),
                        filler: None, marker: format!("§{}§", &cap[1]),
                    });
                }
            }
        }
        fragments
    }
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub index: usize,
    pub total: usize,
    pub content: String,
    pub filler: Option<String>,
    pub marker: String,
}

fn encode_fragment_id(index: usize, total: usize) -> String {
    // Simple encoding: base62-ish compact ID
    format!("{}of{}", index + 1, total)
}

fn decode_fragment_id(id: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = id.split("of").collect();
    if parts.len() == 2 {
        let index = parts[0].parse::<usize>().ok()?.checked_sub(1)?;
        let total = parts[1].parse::<usize>().ok()?;
        Some((index, total))
    } else { None }
}
