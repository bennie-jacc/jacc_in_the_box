use crate::{Game, game_state::GameState, draw_util::draw_screen_header};

use ggez::{*, graphics::{Canvas, Text, Color}, input::keyboard::{KeyInput, KeyCode}, glam::vec2};

pub fn draw_main_menu(game: &mut Game, canvas: &mut Canvas) {
    draw_screen_header("Main menu", &game, canvas);
    
    // Adjusted for text sizes
    let middle_of_screen: f32 = game.get_middle_of_screen_width() - 95.0;

    canvas.draw(
        &Text::new("1. Play!"),
        graphics::DrawParam::default()
            .dest(vec2(middle_of_screen, 125.0))
            .color(Color::GREEN)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("2. How to play?"),
        graphics::DrawParam::default()
            .dest(glam::vec2(middle_of_screen, 225.0))
            .color(Color::CYAN)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("3. Leaderboard"),
        graphics::DrawParam::default()
            .dest(glam::vec2(middle_of_screen, 325.0))
            .color(Color::RED)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("0. Exit!"),
        graphics::DrawParam::default()
            .dest(glam::vec2(middle_of_screen, 425.0))
            .color(Color::MAGENTA)
            .scale([2.0, 2.0])
    );
}

// Key down event, when called from main_menu.
pub fn kde_main_menu(ctx: &mut Context, game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Key1) {
        game.start_game(ctx);
    }
    else if input.keycode == Some(KeyCode::Key2) {
        game.game_state = GameState::HowToPlay;
    }
    else if input.keycode == Some(KeyCode::Key3) {
        game.game_state = GameState::Leaderboard;
    }
    else if input.keycode == Some(KeyCode::Key0) {
        std::process::exit(0);
    }
}