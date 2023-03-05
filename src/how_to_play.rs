use ggez::{input::keyboard::{KeyInput, KeyCode}, graphics::{Canvas, Text, DrawParam, Color}, glam::vec2};

use crate::{Game, draw_util::draw_game_title, game_state::GameState};

const HOW_TO_PLAY: &str = "Welcome to Jacc In The Box!\nThe game consists in waiting for the Jacc in the Box to pop and pressing the SPACE key as fast as you can!\nHave fun and good luck on the leaderboard!";

pub fn draw_how_to_play(game: &mut Game, canvas: &mut Canvas) {
    draw_game_title(&game.name, canvas);

    let mut text: Text = Text::new(HOW_TO_PLAY);

    text.set_bounds(vec2(300.0, 150.0));

    canvas.draw(
        &text,
        DrawParam::default()
            .dest(vec2(150.0, 150.0))
            .color(Color::BLACK)
            .scale([2.0, 2.0])
    );

    let mut text_go_back: Text = Text::new("Press ESC to go back to the main menu!");

    text_go_back.set_bounds(vec2(250.0, 150.0));

    canvas.draw(
        &text_go_back,
        DrawParam::default()
            .dest(vec2(150.0, 400.0))
            .color(Color::CYAN)
            .scale([2.0, 2.0])
    );
}

// Key down event for when Game State is set to how to play. :)
pub fn kde_how_to_play(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) {
        game.game_state = GameState::MainMenu;
    }
}