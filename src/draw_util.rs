use ggez::{graphics::{Canvas, Text, self, Color, DrawParam}, glam::{self, vec2}};

use crate::Game;

pub fn draw_screen_header(header: &str, game: &Game, canvas: &mut Canvas) {
    let text: Text = Text::new(header);

    let dest: glam::Vec2 = glam::vec2(
        (game.get_screen_width() / 2.0) - 60.0,
        25.0
    );

    canvas.draw(&text,
        graphics::DrawParam::default()
            .dest(dest)
            .color(Color::BLACK)
            .scale([2.0, 2.0])
    );
}

pub fn draw_go_to_main_menu(game: &Game, canvas: &mut Canvas) {
    let text: Text = Text::new("Press ESC to go back to the Main Menu!");

    canvas.draw(
        &text,
        DrawParam::default()
            .dest(vec2(
                game.get_middle_of_screen_width() - 175.0,
                game.get_screen_height() - 50.0
            ))
            .color(Color::BLACK)
            .scale([1.25, 1.25])
    );
}