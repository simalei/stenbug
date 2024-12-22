use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use serde::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Default, PartialOrd, PartialEq)]
pub struct Config {
    pub system: System,
    pub search: Search
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct System {
    pub duration: Duration
}

impl Default for System {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(3600),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Search {
    pub query: String,
    pub categories: String,
    pub purity: String,
    pub sorting: String,
    pub order: String,
    pub top_range: String,
    pub at_least: String,
    pub resolutions: String,
    pub ratios: String,
    pub colors: String,
    pub api_key: String,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            query: "+landscape +mountain".to_string(),
            categories: "111".to_string(),
            purity: "100".to_string(),
            sorting: "date_added".to_string(),
            order: "desc".to_string(),
            top_range: "1M".to_string(),
            at_least: "1920x1080".to_string(),
            resolutions: "".to_string(),
            ratios: "".to_string(),
            colors: "".to_string(),
            api_key: "".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(toml::from_str::<Config>(buf.as_str())?)
    }
}