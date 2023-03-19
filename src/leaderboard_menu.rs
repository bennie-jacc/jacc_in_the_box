use ggez::{graphics::{Canvas, DrawParam, Color, Text}, glam::vec2, input::keyboard::{KeyInput, KeyCode}};

use crate::{Game, draw_util::{draw_screen_header, draw_go_to_main_menu}};

pub fn draw_leaderboard(game: &mut Game, canvas: &mut Canvas) {
    draw_screen_header("Leaderboard", &game, canvas);

    // First leaderboard entry Y position.
    let mut height: f32 = 125.0;

    for (ind, entry) in game.leaderboard.leaderboard.iter().enumerate() {
        canvas.draw(
            &Text::new(format!("{}. {}: {}", ind + 1, entry.name, entry.highscore)),
            DrawParam::default()
                .dest(vec2(
                    game.get_middle_of_screen_width() - 75.0,
                    height
                ))
                .color(Color::BLACK)
                .scale([1.25, 1.25])
        );

        height += 40.0;
    }

    draw_go_to_main_menu(game, canvas);
}

pub fn kde_leaderboard(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) { game.go_to_main_menu(); }
}