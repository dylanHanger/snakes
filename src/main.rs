use bevy::{
    core_pipeline::ClearColor,
    prelude::{default, App, Color},
    window::WindowDescriptor,
    MinimalPlugins,
};

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

    app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 600.,
            height: 600.,
            title: "Snakes!".to_string(),
            ..default()
        })
        .add_plugins(MinimalPlugins)
        .add_plugin(SnakesPlugin {
            config_file: cli.config,
        });

    if !cli.headless {
        app.add_plugin(HeadfulPlugin);
    }

    app.run();
}
