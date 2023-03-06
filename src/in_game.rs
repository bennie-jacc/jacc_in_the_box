use ggez::{input::keyboard::KeyInput, graphics::{Canvas, DrawParam, Color, Text}, glam::vec2};
use crate::{Game, draw_util::draw_game_title, jacc::JaccState};

pub fn draw_in_game(game: &mut Game, canvas: &mut Canvas) {
    draw_game_title(&game.name, canvas);

    match game.jacc.get_jacc_state() {
        JaccState::InTheBox      => draw_in_box(game, canvas),
        JaccState::OutOfBox      => draw_out_of_box(game, canvas),
        JaccState::NotApplicable => panic!("Should've never gotten here.")
    }
}

pub fn kde_in_game(game: &mut Game, input: &KeyInput) {
    
}

fn draw_in_box(game: &mut Game, canvas: &mut Canvas) {
    // todo!("Draw clown in the box!");
    
    canvas.draw(
        &Text::new("Press space once the clown pops out of the box!"),
        DrawParam::default()
            .dest(vec2(200.0, 200.0))
            .color(Color::BLACK)
            .scale([2.0, 2.0])
    );

    if game.jacc.get_timer() == game.jacc.get_clown_rng() {
        game.jacc.set_jacc_state_out_of_box();
    }
    else {
        game.jacc.increment_timer();
    }
}

fn draw_out_of_box(game: &mut Game, canvas: &mut Canvas) {
    // todo!("Draw clown out of the box!")

    canvas.draw(
        &Text::new("PRESS SPACE!!!"),
        DrawParam::default()
            .dest(vec2(200.0, 200.0))
            .color(Color::BLACK)
            .scale([4.0, 4.0])
    );
}
