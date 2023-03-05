use crate::{Game, game_state::GameState};

use ggez::{*, graphics::{Canvas, Text, Color}, input::keyboard::{KeyInput, KeyCode}};

pub fn draw_main_menu(game: &mut Game, canvas: &mut Canvas, ctx: &mut Context) {
    draw_game_title(&game.name, canvas);
    draw_menu_options(canvas);
}

pub fn draw_game_title(game_name: &str, canvas: &mut Canvas) {
    let text: Text = Text::new(game_name);
    let dest: glam::Vec2 = glam::vec2(280.0, 25.0);
    canvas.draw(
        &text,
        graphics::DrawParam::default()
            .dest(dest)
            .color(Color::BLACK)
            .scale([2.5, 2.5])
    )
}

pub fn draw_menu_options(canvas: &mut Canvas) {
    canvas.draw(
        &Text::new("1. Play!"),
        graphics::DrawParam::default()
            .dest(glam::vec2(280.0, 75.0))
            .color(Color::GREEN)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("2. How to play?"),
        graphics::DrawParam::default()
            .dest(glam::vec2(280.0, 125.0))
            .color(Color::CYAN)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("3. Leaderboard!"),
        graphics::DrawParam::default()
            .dest(glam::vec2(280.0, 175.0))
            .color(Color::RED)
            .scale([2.0, 2.0])
    );

    canvas.draw(
        &Text::new("0. Exit."),
        graphics::DrawParam::default()
            .dest(glam::vec2(280.0, 225.0))
            .color(Color::YELLOW)
            .scale([2.0, 2.0])
    );

}

// Key down event, when called from main_menu.
pub fn kde_main_menu(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Key1) {
        game.game_state = GameState::InGame;
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