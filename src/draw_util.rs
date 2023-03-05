use ggez::{graphics::{Canvas, Text, self, Color}, glam};

pub fn draw_game_title(game_name: &str, canvas: &mut Canvas) {
    let text: Text = Text::new(game_name);
    let dest: glam::Vec2 = glam::vec2(280.0, 25.0);
    canvas.draw(&text,
        graphics::DrawParam::default()
            .dest(dest)
            .color(Color::BLACK)
            .scale([2.5, 2.5])
    );
}