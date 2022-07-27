use bevy::{
    core_pipeline::ClearColor,
    prelude::{default, App, Color},
    window::WindowDescriptor,
};

mod config;
mod game;

use clap::Parser;
use config::Cli;
use game::SnakesPlugin;

fn main() {
    let cli = Cli::parse();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 600.,
            height: 600.,
            title: "Snakes!".to_string(),
            ..default()
        })
        .add_plugin(SnakesPlugin {
            config_file: cli.config,
        })
        .run();
}
