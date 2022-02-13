use crate::*;
use rand_xoshiro::rand_core::RngCore;

pub fn return_to_move_state(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    keyboard.clear();
    app_state.set(GameState::Move).unwrap();
}

pub fn create_random_tile(
    mut commands: Commands,
    query: Query<&Position, With<Tile>>,
    mut rng: ResMut<Xoshiro256StarStar>,
) {
    print!("create tile!");
    let mut map = vec![false; TILE_NUM];
    for pos in query.iter() {
        map[pos.index()] = true;
    }
    let candidates: Vec<usize> = (0..TILE_NUM).filter(|&i| !map[i]).collect();
    let idx = candidates[rng.next_u64() as usize % candidates.len()];
    let position = Position::from_index(idx);
    let tile = Tile(2);
    commands
        .spawn()
        .insert(tile.clone())
        .insert(position.clone())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::from(tile),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: position.into(),
            ..Default::default()
        });
}
