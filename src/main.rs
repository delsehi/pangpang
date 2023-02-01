use bevy::prelude::*;


fn main() {
    println!("Hello world");

    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                ..default()
            },
            texture: asset_server.load("player_walk.png"),
            sprite: Sprite {
                ..default()
            },
            ..default()
        }, Player
    ));

}

#[derive(Component)]
pub struct Player;