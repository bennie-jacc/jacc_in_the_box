use ggez::{input::keyboard::{KeyInput, KeyCode}, graphics::{Canvas, DrawParam, Color, Text}, glam::vec2, Context};
use crate::{Game, draw_util::draw_game_title, jacc::JaccState, game_state::GameState};

const DESIRED_FPS: u32 = 60;

pub fn draw_in_game(game: &mut Game, canvas: &mut Canvas, ctx: &mut Context) {
    draw_game_title(&game.name, canvas);

    match game.jacc.get_jacc_state() {
        JaccState::InTheBox      => draw_in_box(game, canvas, ctx),
        JaccState::OutOfBox      => draw_out_of_box(game, canvas, ctx),
        JaccState::NotApplicable => panic!("Should've never gotten here.")
    }
}

pub fn kde_in_game(game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Space) {
        game.game_state = match game.jacc.get_jacc_state() {
            JaccState::OutOfBox => GameState::AfterGame(true),
            JaccState::InTheBox => GameState::AfterGame(false),
            JaccState::NotApplicable => panic!("This should've never happened.")
        };
    }
}

fn draw_in_box(game: &mut Game, canvas: &mut Canvas, ctx: &mut Context) {
    // todo!("Draw clown in the box!");
    
    canvas.draw(
        &Text::new("Press space once the clown pops out of the box!"),
        DrawParam::default()
            .dest(vec2(200.0, 200.0))
            .color(Color::BLACK)
            .scale([2.0, 2.0])
    );

    while ctx.time.check_update_time(DESIRED_FPS) {
        if game.jacc.get_timer() == game.jacc.get_clown_rng() {
            game.jacc.set_jacc_state_out_of_box();
        }
        else {
            game.jacc.increment_timer();
        }
    }
}

fn draw_out_of_box(game: &mut Game, canvas: &mut Canvas, ctx: &mut Context) {
    // todo!("Draw clown out of the box!")

    canvas.draw(
        &Text::new("PRESS SPACE!!!"),
        DrawParam::default()
            .dest(vec2(200.0, 200.0))
            .color(Color::BLACK)
            .scale([4.0, 4.0])
    );

    while ctx.time.check_update_time(DESIRED_FPS) {
        game.jacc.increment_time_since_pop();
    }
}