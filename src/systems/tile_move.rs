use crate::*;

#[derive(PartialEq, Eq)]
pub enum MoveEvent {
    Left,
    Right,
    Up,
    Down,
}

pub fn send_move_event(keyboard: Res<Input<KeyCode>>, mut ev_move: EventWriter<MoveEvent>) {
    if keyboard.just_pressed(KeyCode::Left) {
        ev_move.send(MoveEvent::Left)
    } else if keyboard.just_pressed(KeyCode::Right) {
        ev_move.send(MoveEvent::Right)
    } else if keyboard.just_pressed(KeyCode::Up) {
        ev_move.send(MoveEvent::Up)
    } else if keyboard.just_pressed(KeyCode::Down) {
        ev_move.send(MoveEvent::Down)
    }
}

pub fn move_tiles_system(
    mut ev_move: EventReader<MoveEvent>,
    mut query: Query<(&mut Transform, &mut Position), With<Tile>>,
    mut app_state: ResMut<State<GameState>>,
) {
    let mut moved = false;
    for ev in ev_move.iter() {
        let (dx, dy, rot) = match ev {
            MoveEvent::Left => (-1, 0, 0),
            MoveEvent::Right => (1, 0, 2),
            MoveEvent::Up => (0, 1, 3),
            MoveEvent::Down => (0, -1, 1),
        };

        let mut map = vec![0; TILE_NUM];
        for (_, pos) in query.iter() {
            map[pos.index()] = 1;
        }
        for _ in 0..rot {
            rotate_map(&mut map);
        }
        for i in 0..SIDE_LENGTH {
            let mut v = 0;
            for j in 0..SIDE_LENGTH {
                v += 1 - map[i * SIDE_LENGTH + j];
                map[i * SIDE_LENGTH + j] += v - 1;
            }
        }
        for _ in rot..4 {
            rotate_map(&mut map);
        }
        for (mut trans, mut pos) in query.iter_mut() {
            let idx = pos.index();
            pos.x += dx * map[idx];
            pos.y += dy * map[idx];
            *trans = pos.clone().into();
        }
        moved = true;
    }
    if moved {
        app_state.set(GameState::Spawn).unwrap();
    }
}

fn rotate_map(a: &mut [i32]) {
    const ROT: [usize; TILE_NUM] = [3, 7, 11, 15, 11, 6, 10, 14, 14, 10, 10, 13, 15, 13, 14, 15];
    for (i, &j) in ROT.iter().enumerate() {
        a.swap(i, j)
    }
}
