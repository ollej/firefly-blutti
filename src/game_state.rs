pub enum GameState {
    Playing,
    Title,
    Credits,
    Info,
    Died,
    GameOver(bool),
}
