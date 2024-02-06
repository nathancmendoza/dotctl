
extern crate serde;
extern crate serde_yaml;
extern crate shlex;

use serde::{Serialize, Deserialize};

use std::str::FromStr;
use std::io;
use std::process::Command;


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

impl HookSpec {
    pub fn try_run_all(&self) -> io::Result<()> {
        for mut cmd in self.make_commands().into_iter() {
            let hook_output = cmd.output();
            match hook_output {
                Ok(output) => {
                    if output.status.success() {
                        if let Ok(output_str) = String::from_utf8(output.stdout) {
                            println!("Command output:\n {}", output_str);
                        }
                        else {
                            eprintln!("Error converting output to UTF-8");
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Garbage output")))
                        }
                    }
                    else {
                        println!("Command failed with: {:?}", output.status);
                        return Err(io::Error::new(io::ErrorKind::Interrupted, format!("Command finished unsuccessfully")));
                    }
                },
                Err(err) => {
                    eprintln!("Error running command: {:?}", err);
                    return Err(io::Error::new(io::ErrorKind::Interrupted, format!("Command not started")))
                }
            };
        }
        Ok(())
    }

    pub fn execute_time(&self) -> &HookExecutionTime {
        &self.when
    }

    fn make_commands(&self) -> Vec<Command> {
        self.commands.iter()
            .map(| s | shlex::split(s))
            .filter( | maybe_lex | maybe_lex.is_some())
            .map(| real_lex | real_lex.unwrap())
            .map(| tokens | {
                let mut cmd = Command::new(&tokens[0]);
                cmd.args(&tokens[1..]);
                cmd
            })
            .collect()
    }
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
