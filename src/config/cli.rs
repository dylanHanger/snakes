use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short,
        long,
        value_parser,
        help = "The file to read game settings from",
        value_name = "FILE",
        default_value = "config.yaml"
    )]
    pub config: PathBuf,
    #[clap(
        long,
        action = ArgAction::SetTrue,
        help = "Run in headless mode, without any graphical output"
    )]
    pub headless: bool,
}
