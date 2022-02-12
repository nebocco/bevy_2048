use crate::*;

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    create_board(&mut commands);
    for (idx, num) in [(1, 2), (3, 4), (13, 8)] {
        create_tile(&mut commands, num, Position::from_index(idx));
    }
}

pub fn create_tile(commands: &mut Commands, num: u64, position: Position) {
    let tile = Tile(num);
    commands
        .spawn()
        .insert(tile)
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
