use bevy::prelude::*;
use crate::components::Movable;

use crate::components::Enemy;
use crate::components::Speed;
use crate::components::Velocity;


fn spawn_enemies(
    mut commands: Commands, 
    time: Res<Time>,
    asset_server: Res<AssetServer>
    ) {
    if time.elapsed_seconds() > 10_f32 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("key.png"),
            transform: Transform{
                translation: Vec3::new(10.0, 10.0, 2.0),
                ..Default::default()
            },
            ..Default::default()                                                                                                                                           
        })
        .insert(Enemy)
        .insert(Velocity {x: 1.0, y: 0.0})
        .insert(Speed { speed: 5.0})
        .insert(Movable);
    
    }

}


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies);
    }
}