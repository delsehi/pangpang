use crate::player::PlayerPlugin;
use bevy::prelude::*;
mod global_systems;
mod player;
mod components;


fn main() {

    App::new()
        .insert_resource(ClearColor(Color::BLUE))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(global_systems::movable_system)
        .add_system(global_systems::despawn_outside)
        .run();
}

pub fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

}


