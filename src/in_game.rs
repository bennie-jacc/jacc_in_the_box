use ggez::{input::keyboard::{KeyInput, KeyCode}, graphics::{Canvas, DrawParam, Color, Text, Image}, glam::vec2, Context, mint::Point2};
use crate::{Game, draw_util::draw_screen_header, jacc::{JaccState, Jacc}, game_state::GameState, leaderboard_entry::LeaderboardEntry};

pub fn draw_in_game(game: &mut Game, canvas: &mut Canvas) {
    draw_screen_header("On your toes..", &game, canvas);

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
                let score: f32 = jacc.get_time_since_pop() as f32 / ctx.time.fps() as f32;
            
                jacc.set_winner_time(score);
                game.leaderboard.update_highscores(LeaderboardEntry::new(String::from(&game.username), score));
                
                GameState::AfterGame(true)
            },
            JaccState::InTheBox => GameState::AfterGame(false)
        };
    }
}

fn draw_in_box(game: &mut Game, canvas: &mut Canvas) {
    let in_box_image = game.get_assets().get_jacc_in_the_box_image();
    
    canvas.draw(
        in_box_image,
        DrawParam::default()
            .dest(vec2(150.0, 150.0))
            .scale([0.25, 0.25])
    );
    
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
    canvas.draw(
        game.get_assets().get_jacc_out_of_box_image(),
        DrawParam::new()
            .scale([0.25, 0.25])
    );

    canvas.draw(
        &Text::new("PRESS SPACE!!!"),
        DrawParam::default()
            .dest(vec2(200.0, 200.0))
            .color(Color::BLACK)
            .scale([4.0, 4.0])
    );

    game.get_jacc().increment_time_since_pop();
}