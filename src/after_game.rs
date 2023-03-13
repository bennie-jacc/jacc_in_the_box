use ggez::{graphics::{Canvas, Text, DrawParam, Color}, glam::vec2, input::keyboard::{KeyInput, KeyCode}, Context};

use crate::Game;

pub fn draw_after_game(success: bool, game: &mut Game, canvas: &mut Canvas) {
    let text: String = 
        if success { format!("Congratulations! You took {} seconds!", game.get_jacc().get_winner_time()) }
        else { "You lost! Next time be more patient!".to_string() };
    
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

pub fn kde_after_game(ctx: &Context, game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Escape) { game.go_to_main_menu() } 
    else if input.keycode == Some(KeyCode::Space) { game.start_game(ctx); }
}