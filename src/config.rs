
extern crate serde;
extern crate serde_yaml;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;
use resolve_path::PathResolveExt;


use std::fs::File;
use std::io::{self, BufReader};
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
    Soft,
    Hard,
    Copy
}

impl FromStr for HookExecutionTime {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "presetup" | "pre-setup" => Ok(Self::Presetup),
            "postsetup" | "post-setup" => Ok(Self::Postsetup),
            "preteardown" | "pre-teardown" => Ok(Self::Preteardown),
            "postteardown" | "post-teardown" => Ok(Self::Postteardown),
            _ => Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("Expected one of [presetup, postsetup, preteardown, postteardown]. Got {}", s)
                    ))
        }
    }
}

impl FromStr for LinkMode {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Soft" | "soft" => Ok(Self::Soft),
            "Hard" | "hard" => Ok(Self::Hard),
            "Copy" | "copy" => Ok(Self::Copy),
            _ => Err(io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Expected one of: [soft, hard, copy]. Got {}", s)
                        ))
        }
    }
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
