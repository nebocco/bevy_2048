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
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Position> for Transform {
    fn from(pos: Position) -> Self {
        let x = (pos.x as f32 - (SIDE_LENGTH - 1) as f32 / 2.) * (TILE_SIZE * 1.05);
        let y = (pos.y as f32 - (SIDE_LENGTH - 1) as f32 / 2.) * (TILE_SIZE * 1.05);
        Transform::from_xyz(x, y, 0.)
    }
}

#[derive(Component)]
struct Tile(u64);

fn create_tile(commands: &mut Commands, num: u64, position: Position) {
    commands
        .spawn()
        .insert(Tile(num))
        .insert(position.clone())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: position.into(),
            ..Default::default()
        });
}

#[derive(Component)]
struct Background;

fn create_board(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background);
    for i in 0..SIDE_LENGTH as i32 {
        for j in 0..SIDE_LENGTH as i32 {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    transform: Position::new(i, j).into(),
                    ..Default::default()
                })
                .insert(Background);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    create_board(&mut commands);
    for i in 1..SIDE_LENGTH as i32 {
        for j in 1..SIDE_LENGTH as i32 {
            create_tile(&mut commands, 2, Position::new(i, j));
        }
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
        .run();
}
