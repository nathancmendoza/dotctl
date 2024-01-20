
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
use std::fs;

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

pub fn setup_link(link_info: &LinkSpec, link_repo: &str) -> Result<(), io::Error> {
    let true_source = link_info.source.try_resolve_in(link_repo)?;
    let true_target = link_info.target.try_resolve()?;

    println!("Linking: {:?} -> {:?}", true_source, true_target);

    if !true_source.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("No such source path: {:?}", true_source)));
    }

    match link_info.mode {
        LinkMode::Soft => make_soft_link(&true_source, &true_target),
        LinkMode::Hard => fs::hard_link(&true_source, &true_target),
        LinkMode::Copy => make_copy(&true_source, &true_target)
    }
    // 1. fully resolve target and source paths and verify source exists
    // 2. match against link mode
    //      softlink => create the softlink from target -> source
    //      hardlink => create the hardlink from target -> source
    //      copy => copy contents of target into source
}

pub fn teardown_link(link_info: &LinkSpec) {
    // 1. fully resolve target path and verify target exists
    // 2. remove the filesystem object at target
}

fn make_soft_link(source: &Path, target: &Path) -> Result<(), io::Error> {
    if source.is_file() {
        symlink_dir(source, target)
    }
    else if source.is_dir() {
        symlink_file(source, target)
    }
    else {
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Source must be a regular file or directory.")))
    }
}

fn make_copy(source: &Path, target: &Path) -> Result<(), io::Error> {
    if source.is_file() {
        match fs::copy(source, target) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(io::ErrorKind::Interrupted, format!("Did not copy {:?} -> {:?} completely", source, target)))
        }
    }
    else {
        if !target.exists() {
            fs::create_dir(target)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = target.join(entry.file_name());

            make_copy(&entry_path, &dest_path)?;
        }

        Ok(())
    }
}
