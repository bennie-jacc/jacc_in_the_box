use ggez::{input::keyboard::{KeyInput, KeyCode}, graphics::{Canvas, Text, DrawParam, Color}, glam::vec2};

use crate::{Game, draw_util::{draw_screen_header, draw_go_to_main_menu}, game_state::GameState};

const HOW_TO_PLAY: &str = "Welcome to Jacc In The Box! This game consists in waiting for the Jacc in the Box to pop and pressing the SPACE key as fast as you can!\nHave fun and good luck on the leaderboard!";

pub fn draw_how_to_play(game: &mut Game, canvas: &mut Canvas) {
    draw_screen_header("How to play", &game, canvas);

    let mut text: Text = Text::new(HOW_TO_PLAY);

    text.set_bounds(vec2(350.0, 150.0));

    canvas.draw(
        &text,
        DrawParam::default()
            .dest(vec2(game.get_middle_of_screen_width() - 235.0, 150.0))
            .color(Color::BLACK)
            .scale([1.5, 1.5])
    );

    draw_go_to_main_menu(game, canvas);
}

// Key down event for when Game State is set to how to play. :)
pub fn kde_how_to_play(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) {
        game.game_state = GameState::MainMenu;
    }
}