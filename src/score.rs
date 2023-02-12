use bevy::prelude::*;
use crate::components::Score;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score)
        .add_system(display_score);
    }
}

fn display_score(score_query: Query<&Score>, mut query: Query<&mut Text> ) {
    let mut score_text = query.single_mut();
    let score = score_query.single();
    score_text.sections[0].value = format!("Score: {}", score.score);
}

fn setup_score(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "Score: ",
            TextStyle {
                font_size: 50.0,
                font: assets.load("FiraSans-Bold.ttf"),
                color: Color::ORANGE,
                ..Default::default()
            }
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..Default::default()
        })
    )
    .insert(Score{score: 0});


}

