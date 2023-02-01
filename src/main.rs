use bevy::prelude::*;

fn main() {
    println!("Hello world");

    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    game.player.x = 0;
    game.player.y = 0;

    commands.spawn(Camera2dBundle::default());

    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                transform: Transform { ..default() },
                texture: asset_server.load("player_walk.png"),
                sprite: Sprite { ..default() },
                ..default()
            })
            .id(),
    );
}

fn move_player(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    if keyboard_input.pressed(KeyCode::Up) {
        game.player.y += 2;
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.player.y -= 2;
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.player.x -= 2;
    } else if keyboard_input.pressed(KeyCode::Right) {
        game.player.x += 2;
    }
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(game.player.x as f32, game.player.y as f32, 1.0),
        ..default()
    };
}

#[derive(Component, Default)]
pub struct Player {
    entity: Option<Entity>,
    x: usize,
    y: usize,
}

#[derive(Resource, Default)]
pub struct Game {
    player: Player,
}
