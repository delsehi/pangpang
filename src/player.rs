use super::components::{Bullet, Direction, Movable, Player, Speed, Velocity, DespawnOutsideWindow};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(spawn_player)
            .add_system(player_keyboard_system)
            .add_system(fire_bullet_system);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player_walk.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Movable)
        .insert(Speed { speed: 5.0 })
        .insert(Direction { x: 1.0, y: 0.0 })
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn player_keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    mut query_direction: Query<&mut Direction, With<Player>>,
) {
    let mut dir = query_direction.get_single_mut().unwrap();
    if let Ok(mut velocity) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -1.0;
            dir.x = -1.0;
            dir.y = 0.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = 1.0;
            dir.x = 1.0;
            dir.y = 0.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.y = 1.0;
            dir.y = 1.0;
            dir.x = 0.0;
        } else if keyboard_input.pressed(KeyCode::Down) {
            velocity.y = -1.0;
            dir.y = -1.0;
            dir.x = 0.0;
        }
        if keyboard_input.just_released(KeyCode::Left)
            || keyboard_input.just_released(KeyCode::Right)
        {
            velocity.x = 0.0;
        }
        if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::Down)
        {
            velocity.y = 0.0;
        }
    }
}

fn fire_bullet_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    query2: Query<&Direction, With<Player>>,
    query: Query<&Transform, With<Player>>,
) {
    let vel = query2.get_single().unwrap();
    if let Ok(player) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player.translation.x, player.translation.y);
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("bullet.png"),
                    transform: Transform {
                        translation: Vec3::new(x, y, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bullet)
                .insert(Movable)
                .insert(DespawnOutsideWindow)
                .insert(Speed { speed: 7.0 })
                .insert(Velocity { x: vel.x, y: vel.y });
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
