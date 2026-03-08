use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub workspace_dir: String,
    pub formatters: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct LocalConfig {
    pub source: String,
    pub targets: HashMap<String, TargetConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TargetConfig {
    pub out_dir: String,
}

impl GlobalConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}

impl LocalConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}
