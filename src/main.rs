use bevy::prelude::*;

mod components;

mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run()
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
