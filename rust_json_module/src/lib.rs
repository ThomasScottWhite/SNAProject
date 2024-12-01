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
    #[pyo3(get)]
    keywords: HashSet<String>, // Added keywords field
}

impl Topic {
    fn new(name: &str, keywords: HashSet<String>) -> Self {
        Topic {
            name: name.to_string(),
            total_references: 0,
            support_references_per_date: HashMap::new(),
            oppose_references_per_date: HashMap::new(),
            neutral_references_per_date: HashMap::new(),
            keywords, // Initialize keywords
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
    let mut topics = vec![
        Topic::new(
            "Israeli Incursions in Tulkarm (August 2023)",
            vec![
                "IDF",
                "Israeli Defense Forces",
                "Tulkarm incursions",
                "Military operations",
                "West Bank",
                "Nour Shams refugee camp",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "General Military Action Against Palestinian Civilians (August 2023)",
            vec![
                "IDF",
                "Palestinian civilians",
                "West Bank",
                "Military operations",
                "Palestinian Health Ministry",
                "UNRWA",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Palestinian Militant Bomb Detonations",
            vec![
                "Hamas militants",
                "Islamic Jihad",
                "Terrorism",
                "Military installations",
                "Border communities",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Israeli Settler Violence (First Half 2023)",
            vec![
                "West Bank settler violence",
                "OCHA",
                "Border communities",
                "Occupation",
                "Self-determination",
                "UN aid workers",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Journalist Killed",
            vec![
                "Shireen Abu Akleh",
                "Press freedom",
                "Al Jazeera",
                "International Criminal Court",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "October 7th Events",
            vec![
                "Hamas-led Attack on Israel",
                "October 7 attacks",
                "Hamas militants",
                "Islamic Jihad",
                "Terrorism",
                "Border communities",
                "Yahya Sinwar",
                "Mohammed Deif",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Nova Music Festival Massacre",
            vec![
                "Nova Music Festival massacre",
                "Hamas militants",
                "Terrorism",
                "Hostage crisis",
                "Border communities",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Mid-October Events",
            vec![
                "Church of Saint Porphyrius Airstrike (October 19)",
                "Church of Saint Porphyrius",
                "Gaza civilians",
                "Palestinian Health Ministry",
                "IDF",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Biden Condemns Settler Violence (October 25)",
            vec![
                "Joe Biden",
                "West Bank settler violence",
                "International response",
                "Occupation",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Communications Blackout (October 28)",
            vec![
                "Gaza communications blackout",
                "Water cutoff",
                "electricity cutoff",
                "Humanitarian crisis",
                "Palestinian civilians",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Jabalia Refugee Camp Airstrike (October 31)",
            vec![
                "Jabalia refugee camp",
                "Gaza civilians",
                "Palestinian Health Ministry",
                "Military operations",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Al-Shifa Ambulance Airstrike",
            vec![
                "Al-Shifa Hospital",
                "Gaza medical staff",
                "Palestinian Red Crescent",
                "Medical infrastructure",
                "WHO",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "School Attacks During Gaza Invasion",
            vec![
                "UNRWA schools",
                "Gaza civilians",
                "Displaced Palestinians",
                "Military operations",
                "UN aid workers",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "UN Casualty Report (November 3)",
            vec![
                "Palestinian Health Ministry",
                "Gaza Health Ministry",
                "OCHA",
                "UN aid workers",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Al-Quds Hospital Incident (November 10)",
            vec![
                "Al-Quds Hospital",
                "Gaza medical staff",
                "Palestinian Red Crescent",
                "Military operations",
                "Civilian casualties",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Hospital Closures (November 12)",
            vec![
                "Al-Shifa Hospital",
                "Al-Quds Hospital",
                "Gaza medical staff",
                "WHO",
                "Medical infrastructure",
                "Palestinian Health Ministry",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "North Gaza Hospital Evacuations (November 21)",
            vec![
                "WHO",
                "Gaza medical staff",
                "Mass evacuation orders",
                "Medical infrastructure",
                "Palestinian Health Ministry",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Qatar-Brokered Agreement (November 22)",
            vec![
                "Hostage deal negotiations",
                "Qatar mediation",
                "Ceasefire",
                "Hostage families",
                "Joe Biden",
                "Humanitarian aid",
                "Palestinian prisoners",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Ongoing Humanitarian Issues",
            vec![
                "Gaza Strip Infrastructure Crisis",
                "Water/electricity cutoff",
                "Blackout",
                "Gaza civilians",
                "Humanitarian aid",
                "UNRWA",
                "Palestinian Red Crescent",
                "WHO",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
        Topic::new(
            "Hospital System Crisis",
            vec![
                "WHO",
                "Gaza medical staff",
                "Medical infrastructure",
                "Palestinian Health Ministry",
                "MSF",
                "Palestinian Red Crescent",
            ]
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect(),
        ),
    ];

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

            for topic in topics.iter_mut() {
                for keyword in topic.keywords.iter() {
                    if post_text.contains(keyword) {
                        topic.add_reference(post_date.clone(), post_support);
                        break; // Stop checking after finding a matching keyword
                    }
                }
            }
        }
    }

    // Return the list of topics
    Ok(topics)
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
