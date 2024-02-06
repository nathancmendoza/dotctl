
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
