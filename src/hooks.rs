
extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HookSpec {
    commands: Vec<String>,
    when: HookExecutionTime
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum HookExecutionTime {
    PRESETUP,
    POSTSETUP,
    PRETEARDOWN,
    POSTTEARDOWN
}
