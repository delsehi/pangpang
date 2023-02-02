use crate::player::PlayerPlugin;
use bevy::prelude::*;
mod movable_system;
mod player;
mod components;


fn main() {

    App::new()
        .insert_resource(ClearColor(Color::BLUE))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(movable_system::movable_system)
        .run();
}

pub fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

}


