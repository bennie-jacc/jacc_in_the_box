use ggez::{graphics::{Canvas, DrawParam, Color, Text}, glam::vec2, input::keyboard::{KeyInput, KeyCode}};

use crate::{Game, draw_util::draw_game_title};

pub fn draw_leaderboard(game: &mut Game, canvas: &mut Canvas) {
    draw_game_title(&game.name, canvas);

    // First leaderboard entry Y position.
    let mut height: f32 = 75.0;

    for (ind, entry) in game.leaderboard.leaderboard.iter().enumerate() {
        canvas.draw(
            &Text::new(format!("{}. {}: {}", ind + 1, entry.name, entry.highscore)),
            DrawParam::default()
                .dest(vec2(300.0, height))
                .color(Color::BLACK)
                .scale([1.25, 1.25])
        );

        height += 30.0;
    }

    canvas.draw(
        &Text::new("Press ESC to go back to the main menu!"),
        DrawParam::default()
            .dest(vec2(300.0, height + 100.0))
            .color(Color::BLACK)
            .scale([1.25, 1.25])
    );
}

pub fn kde_leaderboard(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) { game.go_to_main_menu(); }
}