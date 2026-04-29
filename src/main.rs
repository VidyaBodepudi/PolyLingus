use std::collections::HashMap;
use clap::{Parser, Subcommand};
use colored::Colorize;

use triolingo::core::registry::TransformRegistry;
use triolingo::core::pipeline::Pipeline;
use triolingo::decoder::{UniversalDecoder, ChainDecoder};
use triolingo::payload::Randomizer;
use triolingo::analysis::report::ReportGenerator;
use triolingo::payload::PayloadFragmenter;

#[derive(Parser)]
#[command(name = "triolingo", version, about = "Universal text transformer, steganography engine & AI red-teaming toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode/transform text
    Encode {
        #[arg(short, long)]
        transform: String,
        input: String,
        #[arg(short, long, value_parser = parse_param)]
        param: Vec<(String, String)>,
    },
    /// Decode text (specify transform or auto-detect)
    Decode {
        #[arg(short, long)]
        transform: Option<String>,
        input: String,
        #[arg(short, long, value_parser = parse_param)]
        param: Vec<(String, String)>,
        /// Try chained decoding up to this depth
        #[arg(long, default_value = "1")]
        depth: usize,
    },
    /// Execute a transform pipeline chain
    Pipeline {
        chain: String,
        input: String,
        #[arg(long)]
        reverse: bool,
    },
    /// Analyze text for hidden content, injections, and attacks
    Analyze {
        input: String,
        #[arg(short, long, default_value = "all")]
        mode: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Apply random transforms per word
    Randomize { input: String },
    /// Fragment a payload into parts
    Fragment {
        input: String,
        #[arg(short, long, default_value = "3")]
        parts: usize,
        /// Interleave with filler text
        #[arg(long)]
        interleave: bool,
    },
    /// Reassemble fragments
    Reassemble { input: String },
    /// Steganography operations
    Stego {
        #[command(subcommand)]
        action: StegoAction,
    },
    /// Estimate token counts across tokenizers
    Tokens { input: String },
    /// List all available transforms
    List {
        #[arg(short, long)]
        category: Option<String>,
    },
}

#[derive(Subcommand)]
enum StegoAction {
    /// Embed a message using steganography
    Embed {
        #[arg(short, long)]
        method: String,
        #[arg(short, long)]
        message: String,
        /// Cover text or image path
        #[arg(short, long)]
        cover: String,
        /// Output path (for image stego)
        #[arg(short, long, default_value = "stego_output.png")]
        output: String,
    },
    /// Extract hidden message
    Extract {
        #[arg(short, long)]
        method: String,
        input: String,
    },
}

fn parse_param(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 { Ok((parts[0].to_string(), parts[1].to_string())) }
    else { Err(format!("Invalid param: '{}'. Use key=value", s)) }
}

fn main() {
    let cli = Cli::parse();
    let registry = TransformRegistry::with_defaults();

    match cli.command {
        Commands::Encode { transform, input, param } => {
            let params: HashMap<String, String> = param.into_iter().collect();
            match registry.get(&transform) {
                Some(t) => match t.encode(&input, &params) {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                },
                None => eprintln!("{}: Unknown transform '{}'. Use 'triolingo list'", "Error".red().bold(), transform),
            }
        }

        Commands::Decode { transform, input, param, depth } => {
            let params: HashMap<String, String> = param.into_iter().collect();
            match transform {
                Some(key) => match registry.get(&key) {
                    Some(t) => match t.decode(&input, &params) {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                    },
                    None => eprintln!("{}: Unknown transform '{}'", "Error".red().bold(), key),
                },
                None => {
                    println!("{}", "=== Universal Decoder ===".cyan().bold());
                    if depth > 1 {
                        println!("{}", format!("Chain depth: {}", depth).dimmed());
                        let results = ChainDecoder::decode(&input, &registry, depth);
                        if results.is_empty() {
                            println!("{}", "No decodings found".yellow());
                        } else {
                            for (i, r) in results.iter().take(10).enumerate() {
                                println!("{}. {}", (i + 1).to_string().green().bold(), r);
                            }
                        }
                    } else {
                        let results = UniversalDecoder::decode(&input, &registry);
                        if results.is_empty() {
                            println!("{}", "No decodings found".yellow());
                        } else {
                            for (i, r) in results.iter().take(5).enumerate() {
                                println!("{}. {}", (i + 1).to_string().green().bold(), r);
                            }
                        }
                    }
                }
            }
        }

        Commands::Pipeline { chain, input, reverse } => {
            let pipeline = Pipeline::from_chain_str("cli_pipeline", &chain);
            match if reverse { pipeline.decode(&input, &registry) } else { pipeline.encode(&input, &registry) } {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
            }
        }

        Commands::Analyze { input, mode, json } => {
            if json {
                match ReportGenerator::to_json(&input) {
                    Ok(j) => println!("{}", j),
                    Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                }
                return;
            }
            println!("{}", "=== TrioLingo Analysis ===".cyan().bold());
            let params = HashMap::new();
            let keys: Vec<&str> = match mode.as_str() {
                "prompt-injection" | "pi" => vec!["prompt_injection_scan"],
                "homoglyph" | "hg" => vec!["homoglyph_detect"],
                "stego-scan" | "ss" => vec!["stego_scan"],
                "entropy" | "en" => vec!["entropy"],
                "unicode" | "us" => vec!["unicode_scan"],
                _ => vec!["prompt_injection_scan", "homoglyph_detect", "stego_scan", "entropy", "unicode_scan"],
            };
            for key in keys {
                if let Some(t) = registry.get(key) {
                    println!("{}", t.encode(&input, &params).unwrap_or_default());
                    println!();
                }
            }
        }

        Commands::Randomize { input } => {
            println!("{}", Randomizer::randomize(&input, &registry));
        }

        Commands::Fragment { input, parts, interleave } => {
            let fragments = PayloadFragmenter::fragment(&input, parts, None);
            if interleave {
                println!("{}", PayloadFragmenter::interleave(&fragments));
            } else {
                for frag in &fragments {
                    println!("{} {}", frag.marker, frag.content);
                }
            }
        }

        Commands::Reassemble { input } => {
            let fragments = PayloadFragmenter::extract_fragments(&input);
            if fragments.is_empty() {
                eprintln!("{}: No fragments found", "Error".red().bold());
            } else {
                println!("{}", PayloadFragmenter::reassemble(&fragments));
            }
        }

        Commands::Stego { action } => {
            match action {
                StegoAction::Embed { method, message, cover, output } => {
                    let mut params = HashMap::new();
                    params.insert("cover".to_string(), cover);
                    params.insert("output".to_string(), output);
                    let key = match method.as_str() {
                        "zwsp" | "zero-width" => "zwsp_stego",
                        "whitespace" | "ws" => "whitespace_stego",
                        "image" | "lsb" => "image_stego",
                        _ => { eprintln!("{}: Unknown method '{}'. Use: zwsp, whitespace, image", "Error".red().bold(), method); return; }
                    };
                    match registry.get(key) {
                        Some(t) => match t.encode(&message, &params) {
                            Ok(r) => println!("{}", r),
                            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                        },
                        None => eprintln!("{}: Transform not found", "Error".red().bold()),
                    }
                }
                StegoAction::Extract { method, input } => {
                    let params = HashMap::new();
                    let key = match method.as_str() {
                        "zwsp" | "zero-width" => "zwsp_stego",
                        "whitespace" | "ws" => "whitespace_stego",
                        _ => { eprintln!("{}: Unknown method", "Error".red().bold()); return; }
                    };
                    match registry.get(key) {
                        Some(t) => match t.decode(&input, &params) {
                            Ok(r) => println!("Extracted: {}", r),
                            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                        },
                        None => eprintln!("{}: Transform not found", "Error".red().bold()),
                    }
                }
            }
        }

        Commands::Tokens { input } => {
            let est = triolingo::tokenizer::glitch_tokens::estimate_tokens(&input);
            println!("{}", "=== Token Estimate ===".cyan().bold());
            println!("  Characters: {}", est.char_count);
            println!("  Words:      {}", est.word_count);
            println!("  GPT-4:      ~{} tokens", est.gpt4_estimate);
            println!("  Claude:     ~{} tokens", est.claude_estimate);
            println!("  Llama:      ~{} tokens", est.llama_estimate);
        }

        Commands::List { category } => {
            let transforms = registry.list_all();
            println!("{} {} transforms available\n",
                "TrioLingo".green().bold(), transforms.len().to_string().cyan().bold());
            let mut by_cat: HashMap<String, Vec<_>> = HashMap::new();
            for info in &transforms {
                let cat = info.category.to_string();
                if let Some(ref filter) = category {
                    if !cat.to_lowercase().contains(&filter.to_lowercase()) { continue; }
                }
                by_cat.entry(cat).or_default().push(info);
            }
            let mut cats: Vec<_> = by_cat.keys().cloned().collect();
            cats.sort();
            for cat in cats {
                let items = &by_cat[&cat];
                println!("  {} ({})", cat.yellow().bold(), items.len());
                for info in items {
                    let rev = if info.reversible { "⟳" } else { "→" };
                    println!("    {} {} {} — {}", rev, info.key.green(), info.name.white().bold(), info.description.dimmed());
                }
                println!();
            }
        }
    }
}
