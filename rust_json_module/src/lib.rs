use chrono::NaiveDateTime;
use pyo3::prelude::*;
use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

#[pyclass]
#[derive(Debug)]
struct Topic {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    total_references: usize,
    #[pyo3(get)]
    support_references_per_date: HashMap<String, usize>,
    #[pyo3(get)]
    oppose_references_per_date: HashMap<String, usize>,
    #[pyo3(get)]
    neutral_references_per_date: HashMap<String, usize>,
}

impl Topic {
    fn new(name: &str, keywords: HashSet<String>) -> Self {
        Topic {
            name: name.to_string(),
            total_references: 0,
            support_references_per_date: HashMap::new(),
            oppose_references_per_date: HashMap::new(),
            neutral_references_per_date: HashMap::new(),
        }
    }

    fn add_reference(&mut self, date: String, support: &str) {
        let counter = match support {
            "supporting" => &mut self.support_references_per_date,
            "opposing" => &mut self.oppose_references_per_date,
            _ => &mut self.neutral_references_per_date,
        };

        *counter.entry(date).or_insert(0) += 1;
        self.total_references += 1;
    }
}

#[derive(Debug, Deserialize)]
struct Post {
    subreddit: String,
    #[serde(deserialize_with = "timestamp_from_string_or_number")]
    timestamp: f64,
    text: String,
}

fn timestamp_from_string_or_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Value::deserialize(deserializer)?;
    match val {
        Value::Number(num) => num
            .as_f64()
            .ok_or_else(|| de::Error::custom("Invalid number for timestamp")),
        Value::String(s) => s
            .parse::<f64>()
            .map_err(|e| de::Error::custom(e.to_string())),
        _ => Err(de::Error::custom("Invalid type for timestamp")),
    }
}

#[pyfunction]
fn process_files(file_paths: Vec<String>) -> PyResult<Vec<Topic>> {
    // Define supporting and opposing subreddits
    let supporting_israel: HashSet<String> = vec![
        "Jewish",
        "Judaism",
        "IsraelUnderAttack",
        "IsraelPalestine",
        "IsraelICYMI",
        "IsraelWar",
        "Israel",
        "IsraelVsHamas",
    ]
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect();

    let opposing_israel: HashSet<String> = vec![
        "Palestine",
        "IsraelPalestine",
        "AskMiddleEast",
        "IsraelHamasWar",
        "islam",
        "israelexposed",
        "exmuslim",
        "IsraelCrimes",
        "PalestinianViolence",
        "AntiSemitismInRedditIsraelWarVideoReport",
        "MuslimLounge",
        "Muslim",
        "Gaza",
        "MuslimCorner",
        "PalestinianvsIsrael",
    ]
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect();

    // Create topics
    let attacks_keywords: HashSet<String> = vec![
        "attack",
        "hospital",
        "bomb",
        "kill",
        "injure",
        "violence",
        "war",
        "conflict",
        "fight",
        "combat",
        "battle",
        "assault",
        "strike",
        "clash",
        "offensive",
        "onslaught",
        "bombard",
        "besiege",
        "invade",
        "raid",
        "beset",
        "pound",
        "blitz",
        "shell",
        "strafe",
        "blow up",
        "destroy",
        "demolish",
        "flatten",
        "level",
        "raze",
        "wreck",
        "ruin",
        "annihilate",
        "exterminate",
        "eradicate",
        "eliminate",
        "extinguish",
        "obliterate",
        "decimate",
        "massacre",
        "butcher",
        "slaughter",
    ]
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect();

    let mut attacks_topic = Topic::new("attacks", attacks_keywords.clone());

    // // Build keyword to topics mapping
    // let mut keyword_to_topics: HashMap<&str, Vec<&mut Topic>> = HashMap::new();
    // for keyword in &attacks_keywords {
    //     keyword_to_topics
    //         .entry(keyword)
    //         .or_insert_with(Vec::new)
    //         .push(&mut attacks_topic);
    // }

    // Process each file
    for file_path in file_paths {
        let file_content = fs::read_to_string(&file_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read file {}: {}",
                file_path, e
            ))
        })?;

        let data: Vec<Post> = serde_json::from_str(&file_content).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse JSON in file {}: {}",
                file_path, e
            ))
        })?;

        for post in data {
            let post_support = determine_supporting(
                &post.subreddit.to_lowercase(),
                &supporting_israel,
                &opposing_israel,
            );
            let post_date = convert_timestamp_to_date(post.timestamp);

            let post_text = post.text.to_lowercase();

            for keyword in attacks_keywords.iter() {
                if post_text.contains(keyword) {
                    attacks_topic.add_reference(post_date.clone(), post_support);
                    break; // Stop checking after finding a matching keyword
                }
            }
        }
    }
    // Return the list of topics
    Ok(vec![attacks_topic])
}

fn determine_supporting(
    subreddit: &str,
    supporting_israel: &HashSet<String>,
    opposing_israel: &HashSet<String>,
) -> &'static str {
    if supporting_israel.contains(subreddit) {
        "supporting"
    } else if opposing_israel.contains(subreddit) {
        "opposing"
    } else {
        "neutral"
    }
}

fn convert_timestamp_to_date(timestamp: f64) -> String {
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
    datetime.format("%Y-%m-%d").to_string()
}

#[pymodule]
fn rust_json_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_files, m)?)?;
    Ok(())
}
