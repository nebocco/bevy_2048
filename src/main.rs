use bevy::prelude::*;

const WINDOW_SIZE: f32 = 500.;
const TILE_SIZE: f32 = 60.;
const SIDE_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn index(&self) -> usize {
        self.y as usize * SIDE_LENGTH + self.x as usize
    }

    fn from_index(i: usize) -> Self {
        Self {
            x: (i % SIDE_LENGTH) as i32,
            y: (i / SIDE_LENGTH) as i32,
        }
    }

    fn to_transform(&self, z: f32) -> Transform {
        let x = (self.x as f32 - (SIDE_LENGTH - 1) as f32 / 2.0) * (TILE_SIZE * 1.05);
        let y = (self.y as f32 - (SIDE_LENGTH - 1) as f32 / 2.0) * (TILE_SIZE * 1.05);
        Transform::from_xyz(x, y, z)
    }
}

impl From<Position> for Transform {
    fn from(pos: Position) -> Self {
        pos.to_transform(10.0)
    }
}

#[derive(Component)]
struct Tile(u64);

impl From<Tile> for Color {
    fn from(Tile(num): Tile) -> Self {
        match num {
            2 => Color::GOLD,
            4 => Color::ORANGE,
            8 => Color::ORANGE_RED,
            _ => Color::RED,
        }
    }
}

fn create_tile(commands: &mut Commands, num: u64, position: Position) {
    commands
        .spawn()
        .insert(Tile(num))
        .insert(position.clone())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::from(Tile(num)),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: position.into(),
            ..Default::default()
        });
}

fn create_board(commands: &mut Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4)),
            ..Default::default()
        },
        ..Default::default()
    });
    for i in 0..SIDE_LENGTH * SIDE_LENGTH {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: Position::from_index(i).to_transform(0.0),
            ..Default::default()
        });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    create_board(&mut commands);
    for (idx, num) in [(1, 2), (3, 4), (13, 8)] {
        create_tile(&mut commands, num, Position::from_index(idx));
    }
}

#[derive(PartialEq, Eq)]
enum MoveEvent {
    Left,
    Right,
    Up,
    Down,
}

fn send_move_event(keyboard: Res<Input<KeyCode>>, mut ev_move: EventWriter<MoveEvent>) {
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

fn move_tiles_system(
    mut ev_move: EventReader<MoveEvent>,
    mut query: Query<(&mut Transform, &mut Position), With<Tile>>,
) {
    for ev in ev_move.iter() {
        let (dx, dy, rot) = match ev {
            MoveEvent::Left => (-1, 0, 0),
            MoveEvent::Right => (1, 0, 2),
            MoveEvent::Up => (0, 1, 3),
            MoveEvent::Down => (0, -1, 1),
        };

        let mut map = vec![0; SIDE_LENGTH * SIDE_LENGTH];
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
    }
}

const ROT: [usize; SIDE_LENGTH * SIDE_LENGTH] =
    [3, 7, 11, 15, 11, 6, 10, 14, 14, 10, 10, 13, 15, 13, 14, 15];

fn rotate_map(a: &mut [i32]) {
    for (i, &j) in ROT.iter().enumerate() {
        a.swap(i, j)
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "2048".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_event::<MoveEvent>()
        .add_system(send_move_event)
        .add_system(move_tiles_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
