
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
        app_conf: String,
    },
    Teardown{
        app_conf: String,
    },
}

