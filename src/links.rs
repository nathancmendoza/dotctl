
extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LinkSpec {
    source: String,
    target: String,
    mode: LinkMode
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum LinkMode {
    SOFTLINK,
    HARDLINK,
    COPY
}
