#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Move,
    Spawn,
    GameOver,
}
