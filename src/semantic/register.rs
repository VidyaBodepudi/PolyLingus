use std::collections::HashMap;
use crate::core::transform::*;

/// Shift text between formal/informal/academic/slang registers
pub struct RegisterShift;

impl Transform for RegisterShift {
    fn info(&self) -> TransformInfo {
        TransformInfo {
            key: "register_shift".into(), name: "Register Shift".into(),
            description: "Convert between formal/informal/academic/slang registers".into(),
            category: TransformCategory::Semantic, reversible: false,
            parameters: vec![ParameterInfo {
                name: "target".into(), description: "formal|informal|academic|slang".into(),
                default_value: "formal".into(),
                param_type: ParamType::Choice(vec!["formal".into(),"informal".into(),"academic".into(),"slang".into()]),
            }],
        }
    }

    fn encode(&self, input: &str, params: &HashMap<String, String>) -> TransformResult {
        let target = params.get("target").map(|s| s.as_str()).unwrap_or("formal");
        let replacements = get_register_map(target);
        let mut result = input.to_string();
        for (from, to) in &replacements {
            result = word_replace(&result, from, to);
        }
        match target {
            "formal" => Ok(expand_contractions(&result)),
            "academic" => {
                let r = expand_contractions(&result);
                if !r.starts_with("It ") && !r.starts_with("The ") {
                    Ok(format!("It can be observed that {}", r.to_lowercase()))
                } else { Ok(r) }
            }
            "slang" => { Ok(result.replace(".", " fr.").replace("!", " no cap!")) }
            _ => Ok(result),
        }
    }

    fn decode(&self, _i: &str, _p: &HashMap<String, String>) -> TransformResult {
        Err(TransformError::Unsupported("Register shift is not reversible".into()))
    }
    fn randomizable(&self) -> bool { false }
}

fn get_register_map(target: &str) -> Vec<(&'static str, &'static str)> {
    match target {
        "formal" => vec![
            ("gonna","going to"),("wanna","want to"),("gotta","have to"),("yeah","yes"),("nope","no"),
            ("ok","acceptable"),("cool","satisfactory"),("big","substantial"),("bad","adverse"),
            ("good","favorable"),("get","obtain"),("use","utilize"),("try","attempt"),("show","demonstrate"),
            ("tell","inform"),("ask","inquire"),("help","assist"),("need","require"),
            ("start","commence"),("end","conclude"),("fix","rectify"),("think","consider"),("give","provide"),
        ],
        "informal" => vec![
            ("going to","gonna"),("want to","wanna"),("have to","gotta"),("yes","yeah"),("no","nope"),
            ("obtain","get"),("utilize","use"),("attempt","try"),("demonstrate","show"),("inform","tell"),
            ("inquire","ask"),("assist","help"),("require","need"),("commence","start"),("conclude","end"),
            ("however","but"),("therefore","so"),("furthermore","also"),
        ],
        "academic" => vec![
            ("show","elucidate"),("think","hypothesize"),("use","employ"),("get","procure"),("try","endeavor"),
            ("help","facilitate"),("tell","articulate"),("look","scrutinize"),("give","furnish"),
            ("big","considerable"),("bad","deleterious"),("good","efficacious"),("start","inaugurate"),
            ("end","terminate"),("find","ascertain"),
        ],
        "slang" => vec![
            ("good","fire"),("great","bussin"),("understand","vibe with"),("agree","bet"),
            ("impressive","goated"),("very","hella"),("friend","fam"),("money","bread"),
            ("excited","hyped"),("cool","lit"),("angry","heated"),("suspicious","sus"),("relax","chill"),
        ],
        _ => vec![],
    }
}

fn word_replace(text: &str, from: &str, to: &str) -> String {
    let lower = text.to_lowercase();
    let from_lower = from.to_lowercase();
    if let Some(pos) = lower.find(&from_lower) {
        let before_ok = pos == 0 || !text.as_bytes()[pos - 1].is_ascii_alphanumeric();
        let end = pos + from.len();
        let after_ok = end >= text.len() || !text.as_bytes()[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return format!("{}{}{}", &text[..pos], to, &text[end..]);
        }
    }
    text.to_string()
}

fn expand_contractions(input: &str) -> String {
    let mut r = input.to_string();
    let list = vec![
        ("don't","do not"),("can't","cannot"),("won't","will not"),("isn't","is not"),
        ("aren't","are not"),("wasn't","was not"),("hasn't","has not"),("haven't","have not"),
        ("didn't","did not"),("wouldn't","would not"),("couldn't","could not"),
        ("shouldn't","should not"),("it's","it is"),("that's","that is"),("there's","there is"),
        ("I'm","I am"),("you're","you are"),("we're","we are"),("they're","they are"),
        ("I've","I have"),("you've","you have"),("I'll","I will"),("you'll","you will"),
    ];
    for (c, e) in &list { r = r.replace(c, e); }
    r
}
