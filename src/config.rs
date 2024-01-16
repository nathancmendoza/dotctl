
extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};

use crate::links::LinkSpec;
use crate::hooks::HookSpec;

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
