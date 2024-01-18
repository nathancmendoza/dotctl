
extern crate serde;
extern crate serde_yaml;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;
use resolve_path::PathResolveExt;

use crate::links::LinkSpec;
use crate::hooks::HookSpec;

use std::fs::File;
use std::io::BufReader;
use std::fmt;

const CONFIG_FILE: &str = "~/.dotter";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DotfileConfiguration {
    configurations: Vec<ConfigSpec>,
    options: DotfileOptions
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DotfileOptions {
    repository: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigSpec {
    name: String,
    os: String,
    status: ConfigStatus,
    links: Vec<LinkSpec>,
    hooks: Option<Vec<HookSpec>>
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
    let file = match File::open(CONFIG_FILE.resolve()) {
        Ok(the_file) => the_file,
        Err(_) => return Err(ConfigurationError::NoConfigurationFound)
    };

    let reader = BufReader::new(file);

    match from_reader(reader) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(ConfigurationError::ParsingError(e))
    }
}

impl DotfileConfiguration {
    pub fn all_configs(&self) -> impl Iterator<Item = &ConfigSpec> {
        self.configurations.iter()
    }
}

impl ConfigSpec {
    pub fn assigned_name(&self) -> &String {
        &self.name
    }

    pub fn config_os(&self) -> &String {
        &self.os
    }

    pub fn config_status(&self) -> &ConfigStatus {
        &self.status
    }
}
