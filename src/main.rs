pub mod game_state;
pub mod main_menu;
pub mod in_game;
pub mod record_view;
pub mod how_to_play;

use ggez::{
    ContextBuilder, Context, event::{EventHandler, self}, GameResult, graphics::{self, Color, Canvas}, input::keyboard::{KeyInput}, GameError
};

use game_state::GameState;
use main_menu::{draw_main_menu, kde_main_menu};
use in_game::draw_in_game;
use record_view::draw_record_view;
use how_to_play::{draw_how_to_play, kde_how_to_play};

pub fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("jacc_in_the_box", "Bernardo")
        .build()
        .expect("Something went wrong creating the game context!");

    let game: Game = Game::new(&mut ctx);
    
    ctx.gfx.set_window_title(&game.name);

    event::run(ctx, event_loop, game);
}

pub struct Game {
    name: String,
    game_state: game_state::GameState
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load and or create resources here..
        Game { 
            name: String::from("Jacc in the Box"),
            game_state: game_state::GameState::MainMenu
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code goes here..
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        match self.game_state {
            game_state::GameState::MainMenu      => draw_main_menu(self, &mut canvas, ctx),
            game_state::GameState::HowToPlay     => draw_how_to_play(),
            game_state::GameState::InGame        => draw_in_game(),
            game_state::GameState::Leaderboard   => draw_record_view()
        };
        
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        match self.game_state {
            GameState::MainMenu  => kde_main_menu(self, &input),
            GameState::HowToPlay => kde_how_to_play(self, &input),
            _ => todo!()
        }

        Ok(())
    }
}