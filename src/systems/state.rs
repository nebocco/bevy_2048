use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Move,
    Spawn,
    GameOver,
}

pub fn change_state(mut ev_state: EventReader<GameState>, mut app_state: ResMut<State<GameState>>) {
    if let Some(state) = ev_state.iter().next() {
        app_state.set(state.clone()).unwrap();
    }
}
