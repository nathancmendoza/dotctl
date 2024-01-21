
extern crate serde;
extern crate serde_yaml;
extern crate symlink;
extern crate resolve_path;

use serde::{Serialize, Deserialize};
use symlink::{symlink_auto, remove_symlink_auto};
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

impl LinkSpec {
    pub fn new(src: &str, dest: &str, mode: LinkMode) -> Self {
        LinkSpec {
            source: src.to_string(),
            target: dest.to_string(),
            mode: mode
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

pub fn setup_link<P: AsRef<Path>>(link_info: &LinkSpec, link_repo: P) -> io::Result<()> {
    // 1. fully resolve target and source paths and verify source exists
    // 2. match against link mode
    //      softlink => create the softlink from target -> source
    //      hardlink => create the hardlink from target -> source
    //      copy => copy contents of target into source

    let true_source = link_info.source.try_resolve_in(link_repo.as_ref())?;
    let true_target = link_info.target.try_resolve()?;

    println!("Linking: {:?} -> {:?}", true_source, true_target);

    if !true_source.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("No such source path: {:?}", true_source)));
    }

    match link_info.mode {
        LinkMode::Soft => symlink_auto(&true_source, &true_target),
        LinkMode::Hard => fs::hard_link(&true_source, &true_target),
        LinkMode::Copy => make_copy(&true_source, &true_target)
    }
}

pub fn teardown_link(link_info: &LinkSpec) -> io::Result<()> {
    // 1. fully resolve target path and verify target exists
    // 2. remove the filesystem object at target
    let true_target = link_info.target.try_resolve()?;

    println!("Removing: {:?}", true_target);

    if !true_target.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("No such target path: {:?}", true_target)));
    }

    if true_target.is_symlink() {
        remove_symlink_auto(true_target)
    }
    else if true_target.is_dir() {
        fs::remove_dir_all(true_target)
    }
    else {
        fs::remove_file(true_target)
    }
}

fn make_copy(source: &Path, target: &Path) -> Result<(), io::Error> {
    if source.is_file() {
        match fs::copy(source, target) {
            Ok(_) => Ok(()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Interrupted, format!("Did not copy {:?} -> {:?} completely", source, target)))
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
