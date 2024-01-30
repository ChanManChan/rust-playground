use bevy::prelude::*;

use crate::game::score::resources::HighScores;
use crate::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    high_scores: Res<HighScores>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    let current_score = high_scores.scores.last().unwrap().1;
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Final Score: {}", current_score.to_string());
    }
}