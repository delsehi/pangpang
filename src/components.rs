use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Score {
    pub score: i32
}


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Speed {
    pub speed: f32
}


#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct DespawnOutsideWindow;