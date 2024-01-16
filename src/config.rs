
extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;

use crate::links::LinkSpec;
use crate::hooks::HookSpec;

use std::fs::File;
use std::io::BufReader;
use std::fmt;

const CONFIG_FILE: &str = "test.yaml";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DotfileConfiguration {
    configurations: Vec<ConfigSpec>,
    options: DotfileOptions
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct DotfileOptions {
    repository: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ConfigSpec {
    name: String,
    system: SystemName,
    status: ConfigStatus,
    links: Vec<LinkSpec>,
    hooks: Option<Vec<HookSpec>>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SystemName {
    LINUX,
    MACOS,
    WINDOWS
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum ConfigStatus {
    READY,
    UNUSED,
}

#[derive(Debug)]
pub enum ConfigurationError {
    ParsingError(serde_yaml::Error),
    NoConfigurationFound,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ConfigurationError::ConfigurationModified => write!(f, "The configuration is dirty"),
            ConfigurationError::NoConfigurationFound => write!(f,"Configuration file not found"),
            ConfigurationError::ParsingError(e) => write!(f, "{}", format!("The configuration has errors: {}", e))
        }
    }
}

impl std::error::Error for ConfigurationError {}

pub fn read_config() -> Result<DotfileConfiguration, ConfigurationError> {
    let file = match File::open(CONFIG_FILE) {
        Ok(the_file) => the_file,
        Err(_) => return Err(ConfigurationError::NoConfigurationFound)
    };

    let reader = BufReader::new(file);

    match from_reader(reader) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(ConfigurationError::ParsingError(e))
    }
}
