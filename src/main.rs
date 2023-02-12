use crate::player::PlayerPlugin;
use crate::enemy::EnemyPlugin;
use crate::score::ScorePlugin;
use bevy::prelude::*;
mod global_systems;
mod player;
mod score;
mod components;
mod enemy;


fn main() {

    App::new()
        .insert_resource(ClearColor(Color::BLUE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Pang pang"),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(setup)
        .add_system(global_systems::movable_system)
        .add_system(global_systems::enemy_shot)
        .add_system(global_systems::despawn_outside)
        .add_system(global_systems::player_hit)
        .run();
}

pub fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

}


