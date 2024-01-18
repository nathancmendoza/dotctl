
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
    Setup{
        app_conf: String,

        #[arg(short, long, default_value_t = false)]
        dry_run: bool
    },
    Teardown{
        app_conf: String,

        #[arg(short, long, default_value_t = false)]
        dry_run: bool
    },
    Status{
        app_conf: String
    },
    Describe{
        app_conf: String,

        #[arg(short, long, default_value_t = false)]
        links_only: bool,

        #[arg(short, long, default_value_t = false)]
        hooks_only: bool
    }
}

