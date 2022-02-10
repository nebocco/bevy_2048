use crate::*;
use bevy::app::AppExit;

pub fn check_game_over(query: Query<&Position, With<Tile>>, mut state: ResMut<State<GameState>>) {
    let mut map = vec![false; TILE_NUM];
    for pos in query.iter() {
        map[pos.index()] = true;
    }
    if (0..TILE_NUM).all(|i| map[i]) {
        state.set(GameState::GameOver).unwrap();
    }
}

pub fn end_game(mut exit: EventWriter<AppExit>) {
    println!("GAME OVER!");
    exit.send(AppExit);
}
