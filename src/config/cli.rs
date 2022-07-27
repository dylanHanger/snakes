use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short,
        long,
        parse(from_os_str),
        help = "The file to read game settings from",
        value_name = "FILE",
        default_value = "config.yaml"
    )]
    pub config: PathBuf,
}
