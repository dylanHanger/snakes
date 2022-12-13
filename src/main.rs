use bevy::{prelude::App, MinimalPlugins};

mod config;
mod game;
mod gui;

use clap::Parser;
use config::Cli;
use game::SnakesPlugin;
use gui::GuiPlugin;

fn main() {
    let cli = Cli::parse();

    let mut app = App::new();

    app.add_plugins(MinimalPlugins).add_plugin(SnakesPlugin {
        config_file: cli.config,
    });

    if !cli.headless {
        app.add_plugin(GuiPlugin);
    }

    app.run();
}
