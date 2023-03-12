use ggez::{input::keyboard::{KeyInput, KeyCode}, graphics::{Canvas, DrawParam, Color, Text}, glam::vec2, Context};
use crate::{Game, draw_util::draw_game_title, jacc::{JaccState, Jacc}, game_state::GameState};

pub fn draw_in_game(game: &mut Game, canvas: &mut Canvas) {
    draw_game_title(&game.name, canvas);

    match game.get_jacc().get_jacc_state() {
        JaccState::InTheBox      => draw_in_box(game, canvas),
        JaccState::OutOfBox      => draw_out_of_box(game, canvas)
    }
}

pub fn kde_in_game(ctx: &Context, game: &mut Game, input: &KeyInput) {
    if input.keycode == Some(KeyCode::Space) {
        game.game_state = match game.get_jacc().get_jacc_state() {
            JaccState::OutOfBox => {
                let jacc: &mut Jacc = game.get_jacc();
                jacc.set_winner_time(jacc.get_time_since_pop() as f32 / ctx.time.fps() as f32);
                
                GameState::AfterGame(true)
            },
            JaccState::InTheBox => GameState::AfterGame(false)
        };
    }
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

    if game.get_jacc().get_timer() == game.get_jacc().get_clown_rng() { game.get_jacc().set_jacc_state_out_of_box(); }
    else { game.get_jacc().increment_timer(); }
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

    game.get_jacc().increment_time_since_pop();
}