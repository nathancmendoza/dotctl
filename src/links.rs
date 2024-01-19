
extern crate serde;
extern crate serde_yaml;
extern crate symlink;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use symlink::{symlink_dir, symlink_file, remove_symlink_file, remove_symlink_dir};
use resolve_path::PathResolveExt;
use std::path::Path;
use std::io;
use std::str::FromStr;

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

pub fn setup_link(link_info: &LinkSpec) {
    // 1. fully resolve target and source paths and verify source exists
    // 2. match against link mode
    //      softlink => create the softlink from target -> source
    //      hardlink => create the hardlink from target -> source
    //      copy => copy contents of target into source
    unimplemented!()
}

pub fn teardown_link(link_info: &LinkSpec) {
    // 1. fully resolve target path and verify target exists
    // 2. remove the filesystem object at target
}
