
extern crate serde;
extern crate serde_yaml;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;
use resolve_path::PathResolveExt;

use crate::links::LinkSpec;
use crate::hooks::HookSpec;

use std::fs::File;
use std::io::{self, BufReader};
use std::iter;
use std::str::FromStr;

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
    Ready,
    Unused,
}

impl FromStr for ConfigStatus {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ready" | "Ready" => Ok(Self::Ready),
            "unused" | "Unused" => Ok(Self::Unused),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Expected one of: [ready, unused]. Got {}", s)))
        }
    }
}

impl DotfileConfiguration {
    pub fn all_configs(&self) -> impl Iterator<Item = &ConfigSpec> {
        self.configurations.iter()
    }

    pub fn config_repo(&self) -> &String {
        &self.options.repository
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

    pub fn links(&self) -> std::slice::Iter<'_, LinkSpec> {
        self.links.iter()
    }

    pub fn hooks(&self) -> Option<std::slice::Iter<'_, HookSpec>> {
        self.hooks.as_ref().map(| h | h.iter())
    }
}

pub fn read_config() -> Result<DotfileConfiguration, io::Error> {
    let file = match File::open(CONFIG_FILE.resolve()) {
        Ok(the_file) => the_file,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, format!("The configuration file was not found")))
    };

    let reader = BufReader::new(file);

    match from_reader(reader) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("The configuration file has error(s): {}", e)))
    }
}
