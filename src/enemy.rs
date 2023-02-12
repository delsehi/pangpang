use bevy::prelude::*;
use rand::thread_rng;
use std::time::Duration;
use crate::components::Movable;
use crate::components::Enemy;
use crate::components::Player;
use crate::components::Speed;
use crate::components::Velocity;
use rand::prelude::Rng;

fn spawn_enemies(
    mut commands: Commands, 
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    asset_server: Res<AssetServer>
    ) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        let mut rng = thread_rng();
        let random_time: f32 = rng.gen_range(4f32..10f32);
        let random_x = rng.gen_range(-5000f32..5000f32);
        let random_y = rng.gen_range(-5000f32..5000f32);
        commands.spawn(SpriteBundle {
            texture: asset_server.load("key.png"),
            transform: Transform{
                translation: Vec3::new(random_x, random_y, 2.0),
                ..Default::default()
            },
            ..Default::default()                                                                                                                                           
        })
        .insert(Enemy)
        .insert(Velocity {x: 1.0, y: 0.0})
        .insert(Speed { speed: random_time})
        .insert(Movable);
    
    }

}

#[derive(Resource)]
struct EnemyTimer {
    timer: Timer
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Velocity, &Transform, With<Enemy>)>) {
        let player_transform = player_query.get_single().unwrap();
        for (mut enemy_velocity, enemy_transform, _enemy) in enemy_query.iter_mut() {
            if enemy_transform.translation.y > player_transform.translation.y {
                enemy_velocity.y = -1_f32;
            } else {
                enemy_velocity.y = 1_f32;
            }
            if enemy_transform.translation.x > player_transform.translation.x {
                enemy_velocity.x = -1_f32;
            } else {
                enemy_velocity.x = 1_f32;
            }

        }

}


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies)
        .add_system(follow_player)
        .insert_resource(EnemyTimer {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating)
        });
    }
}