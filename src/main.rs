use bevy::{prelude::App, MinimalPlugins};

mod config;
mod game;
mod headful;

use clap::Parser;
use config::Cli;
use game::SnakesPlugin;
use headful::HeadfulPlugin;

fn main() {
    let cli = Cli::parse();

    let mut app = App::new();

    app.add_plugins(MinimalPlugins).add_plugin(SnakesPlugin {
        config_file: cli.config,
    });

    if !cli.headless {
        app.add_plugin(HeadfulPlugin);
    }

    app.run();
}
