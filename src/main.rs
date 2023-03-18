pub mod game_state;
pub mod main_menu;
pub mod in_game;
pub mod how_to_play;
pub mod draw_util;
pub mod jacc;
pub mod after_game;
pub mod leaderboard;
pub mod leaderboard_entry;
pub mod leaderboard_menu;
pub mod assets;

use std::{fs::File, env, path};

use ggez::{
    ContextBuilder, Context, event::{EventHandler, self}, GameResult, graphics::{Color, Canvas}, input::keyboard::KeyInput, GameError, conf::{Conf, self}, mint::Point2
};

use jacc::Jacc;
use assets::Assets;
use game_state::GameState;
use leaderboard::Leaderboard;
use leaderboard_menu::{draw_leaderboard, kde_leaderboard};
use main_menu::{draw_main_menu, kde_main_menu};
use in_game::{draw_in_game, kde_in_game};
use how_to_play::{draw_how_to_play, kde_how_to_play};
use after_game::{draw_after_game, kde_after_game};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    generate_toml_file().expect("Failed to generate conf.toml file!");

    // Add a directory for files!
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    }
    else { path::PathBuf::from("./resources") };

    let context_builder = ContextBuilder::new("JaccInTheBox", "Benny")
        .window_setup(conf::WindowSetup::default().title("Jacc In The Box!"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 640.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = context_builder.build()
        .expect("Something went wrong creating the game context!");

    let username = if let Some(val) = args.get(1) { String::from(val) } else { String::from("Player") };

    let game: Game = Game::new(&ctx, username);
    
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
    screen_width: f32,
    screen_height: f32,
    assets: Assets,
    username: String,
    game_state: GameState,
    jacc: Option<Jacc>,
    leaderboard: Leaderboard
}

impl Game {

    // Load and or create resources here..
    pub fn new(ctx: &Context, username: String) -> Game {

        let assets: Assets = Assets::new(ctx).expect("Unable to parse Assets. Please validate if they exist at the pre-defined paths");

        Game { 
            name: String::from("Jacc in the Box"),
            screen_width: 600.0,
            screen_height: 800.0,
            assets,
            game_state: GameState::MainMenu,
            jacc: None,
            username,
            leaderboard: Leaderboard::new()
        }
    }

    pub fn go_to_main_menu(&mut self) {
        self.game_state = GameState::MainMenu;
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

    pub fn get_leaderboard(&mut self) -> &mut Leaderboard { &mut self.leaderboard }

    pub fn get_screen_size(&self) -> Point2<f32> { Point2 { x: self.screen_width, y: self.screen_height } }
    
    pub fn get_screen_width(&self) -> f32 { self.screen_width }

    pub fn get_screen_height(&self) -> f32 { self.screen_height }

    pub fn get_assets(&mut self) -> &mut Assets { &mut self.assets }
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
            GameState::Leaderboard              => draw_leaderboard(self, &mut canvas)
        };
        
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        match self.game_state {
            GameState::MainMenu                  => kde_main_menu(ctx, self, &input),
            GameState::InGame                    => kde_in_game(ctx, self, &input),
            GameState::HowToPlay                 => kde_how_to_play(self, &input),
            GameState::AfterGame(_success) => kde_after_game(ctx, self, &input),
            GameState::Leaderboard               => kde_leaderboard(self, &input)
        }

        Ok(())
    }
}