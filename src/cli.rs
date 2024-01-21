
use clap::{Subcommand, Parser};
use crate::links::LinkMode;

#[derive(Parser)]
pub struct DotterInvocation {

    #[command(subcommand)]
    pub action: DotterActionWord,

}

#[derive(Subcommand)]
pub enum DotterActionWord {
    Use{
        config_path: String,
    },
    Setup{
        app_conf: String,
    },
    Teardown{
        app_conf: String,
    },
}

