
use clap::{Subcommand, Parser};

#[derive(Parser)]
pub struct DotctlInvocation {

    #[command(subcommand)]
    pub action: DotctlActionWord,

}

#[derive(Subcommand)]
pub enum DotctlActionWord {
    Use{
        config_path: String,
    },
    Setup{
        application: String,
    },
    Teardown{
        application: String,
    },
}

