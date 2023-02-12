use crate::components::{Bullet, Enemy, Score};

use super::components::{DespawnOutsideWindow, Movable, Speed, Velocity};
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub fn movable_system(
    mut query: Query<(&Velocity, &mut Transform, &Movable, &Speed)>,
) {
    for (velocity, mut transform, _movable, speed) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * speed.speed;
        translation.y += velocity.y * speed.speed;
    }
}

pub fn despawn_outside(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform, &DespawnOutsideWindow)>,
) {
    let window = windows.get_primary().unwrap();
    let (height, width) = (window.height(), window.width());
    for (entity, transform, _despawn) in query.iter() {
        let t = transform.translation;
        if t.x > width / 2.0 || t.y > height {
            commands.entity(entity).despawn();
        }
    }
}


pub fn enemy_shot (
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut score_query: Query<&mut Score>
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let has_collided = collide(
                bullet_transform.translation,
                Vec2 { x: 10.0, y: 10.0 },
                enemy_transform.translation,
                Vec2 { x: 10.0, y: 10.0 }
            );
            if let Some(_) = has_collided {
                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();
                let mut score = score_query.get_single_mut().unwrap();
                score.score += 1_u32;
            }
        }

    }

}