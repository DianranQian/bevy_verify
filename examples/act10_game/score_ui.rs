use bevy::prelude::*;

use crate::resources::Score;

#[derive(Component)]
pub struct ScoreDisplay;

pub fn setup_score_display(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        },
        ScoreDisplay,
    ));
}

pub fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in &mut query {
        text.0 = format!("Score: {}", score.total);
    }
}
