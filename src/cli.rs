
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

        #[arg(short, long, default_value_t = false)]
        link_config: bool
    },
    Setup{
        app_conf: String,

        #[arg(short, long, default_value_t = false)]
        dry_run: bool,

        #[arg(short, long)]
        link_mode: Option<LinkMode>
    },
    Teardown{
        app_conf: String,

        #[arg(short, long, default_value_t = false)]
        dry_run: bool
    },
}

