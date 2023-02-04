use super::components::{DespawnOutsideWindow, Movable, Speed, Velocity};
use bevy::prelude::*;

pub fn movable_system(
    // mut commands: Commands,
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
