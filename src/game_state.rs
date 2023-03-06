pub enum GameState {
    MainMenu,
    InGame,
    HowToPlay,
    AfterGame(bool),
    Leaderboard
}