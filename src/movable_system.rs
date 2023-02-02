use bevy::prelude::*;
use super::components::{Velocity, Movable };

pub fn movable_system(
    // mut commands: Commands,
    mut query: Query<( &Velocity, &mut Transform, &Movable)>
) {
    for (velocity, mut transform, _movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y
    }
}