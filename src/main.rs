pub mod game_state;
pub mod main_menu;
pub mod in_game;
pub mod how_to_play;
pub mod draw_util;
pub mod jacc;
pub mod after_game;

use std::fs::File;

use ggez::{
    ContextBuilder, Context, event::{EventHandler, self}, GameResult, graphics::{Color, Canvas}, input::keyboard::KeyInput, GameError, conf::Conf
};

use jacc::Jacc;
use game_state::GameState;
use main_menu::{draw_main_menu, kde_main_menu};
use in_game::{draw_in_game, kde_in_game};
use how_to_play::{draw_how_to_play, kde_how_to_play};
use after_game::{draw_after_game, kde_after_game};

pub fn main() {
    generate_toml_file().expect("Failed to generate conf.toml file!");

    let (ctx, event_loop) = ContextBuilder::new("jacc_in_the_box", "Bernardo")
        .build()
        .expect("Something went wrong creating the game context!");

    let game: Game = Game::new();
    
    ctx.gfx.set_window_title(&game.name);

    event::run(ctx, event_loop, game);
}

pub fn generate_toml_file() -> GameResult {
    let conf: Conf = Conf::new();
    let mut config_file = File::create("conf.toml")?;
    conf.to_toml_file(&mut config_file)
}

pub struct Game {
    name: String,
    game_state: GameState,
    jacc: Option<Jacc>
}

impl Game {

    // Load and or create resources here..
    pub fn new() -> Game {
        Game { 
            name: String::from("Jacc in the Box"),
            game_state: game_state::GameState::MainMenu,
            jacc: None
        }
    }

    pub fn start_game(&mut self, ctx: &Context) {
        self.game_state = GameState::InGame;
        self.set_jacc(Jacc::new(ctx));
    }

    pub fn get_jacc(&mut self) -> &mut Jacc {
        match &mut self.jacc {
            Some(jacc) => jacc,
            None => panic!("Jacc wasn't ready at main::get_jacc()")
        }
    }

    pub fn set_jacc(&mut self, jacc: Jacc) { self.jacc = Some(jacc); }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code goes here..
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::WHITE);

        match self.game_state {
            GameState::MainMenu                 => draw_main_menu(self, &mut canvas),
            GameState::HowToPlay                => draw_how_to_play(self, &mut canvas),
            GameState::InGame                   => draw_in_game(self, &mut canvas),
            GameState::AfterGame(success) => draw_after_game(success, self, &mut canvas),
            _ => todo!()
        };
        
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        match self.game_state {
            GameState::MainMenu                  => kde_main_menu(ctx, self, &input),
            GameState::InGame                    => kde_in_game(ctx, self, &input),
            GameState::HowToPlay                 => kde_how_to_play(self, &input),
            GameState::AfterGame(_success) => kde_after_game(ctx, self, &input),
            _ => todo!()
        }

        Ok(())
    }
}