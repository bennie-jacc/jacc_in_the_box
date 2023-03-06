use ggez::{graphics::{Canvas, Text, DrawParam, Color}, glam::vec2, input::keyboard::{KeyInput, KeyCode}};

use crate::{Game, game_state::GameState};

pub fn draw_after_game(success: bool, game: &mut Game, canvas: &mut Canvas) {
    let text: String = if success { format!("Congratulations! You took {} seconds!", game.jacc.get_time_since_pop()) } else { "You lost! Next time be more patient!".to_string() };
    
    canvas.draw(
        &Text::new(text),
        DrawParam::default()
            .dest(vec2(175.0, 200.0))
            .color(Color::BLACK)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new(format!("Please press ESC key to go back to main menu or SPACE to play again!")),
        DrawParam::default()
            .dest(vec2(50.0, 400.0))
            .color(Color::BLACK)
            .scale([1.5, 1.5])
    );
}

pub fn kde_after_game(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) { game.game_state = GameState::MainMenu; } 
    else if input.keycode == Some(KeyCode::Space) { game.reset_game(); }
}