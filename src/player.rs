use super::components::{Player, Velocity, Movable};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system(player_keyboard_system)
            .add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle{
        texture: asset_server.load("player_walk.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Movable)
    .insert(Velocity {x: 0.0, y: 0.0});
}

fn player_keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -1.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = 1.0
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.y = 1.;
        } else if keyboard_input.pressed(KeyCode::Down) {
            velocity.y = -1.;
        }
        if keyboard_input.just_released(KeyCode::Left) || keyboard_input.just_released(KeyCode::Right) {
            velocity.x = 0.;
        }
        if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::Down) {
            velocity.y = 0.0;
        }
    }
}

#[derive(Resource)]
pub struct PlayerState;

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState
    }
}
