
extern crate serde;
extern crate serde_yaml;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;
use resolve_path::PathResolveExt;


use std::fs::File;
use std::io::{self, BufReader};

pub const CONFIG_FILE: &str = "~/.dotctl";

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
pub enum ConfigStatus {
    Active,
    Archived,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HookSpec {
    commands: Vec<String>,
    when: HookExecutionTime
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum HookExecutionTime {
    Presetup,
    Postsetup,
    Preteardown,
    Postteardown
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LinkSpec {
    source: String,
    target: String,
    mode: LinkMode
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum LinkMode {
    Link,
    Copy
}

#[derive(Debug)]
pub enum AppconfSearchError {
    NoneFound,
    MultipleFound
}

impl std::fmt::Display for AppconfSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppconfSearchError::NoneFound => write!(f, "No matching element found"),
            AppconfSearchError::MultipleFound => write!(f, "Multiple matching elements found"),
        }
    }
}

impl std::error::Error for AppconfSearchError {}


impl DotfileConfiguration {

    pub fn read_config() -> io::Result<DotfileConfiguration> {
        let file = File::open(CONFIG_FILE.try_resolve()?)?;
        let reader = BufReader::new(file);

        match from_reader(reader) {
            Ok(conf) => Ok(conf),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("The configuration file has error(s): {}", e)))
        }
    }

    pub fn find_appconf(&self, appname: &String) -> Result<&ConfigSpec, AppconfSearchError> {
        let mut found_index = None;
        for (index, config) in self.configurations.iter().enumerate() {
            if config.name == *appname {
                if found_index. is_some() {
                    return Err(AppconfSearchError::MultipleFound);
                }
                found_index = Some(index);
            }
        }
        found_index
            .map(|index| &self.configurations[index])
            .ok_or(AppconfSearchError::NoneFound)
    }

    pub fn config_repo(&self) -> &String {
        &self.options.repository
    }

}
