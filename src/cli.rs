
use clap::{Subcommand, Parser};

#[derive(Parser)]
pub struct DotterInvocation {

    #[command(subcommand)]
    pub action: DotterActionWord,

}

#[derive(Subcommand)]
pub enum DotterActionWord {
    Use{
        config_path: String,

        #[arg(short, long, default_value_t = false)]
        link_config: bool
    },
    Setup,
    Teardown,
    Status,
    Describe
}

