use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn index(&self) -> usize {
        self.y as usize * SIDE_LENGTH + self.x as usize
    }

    pub fn from_index(i: usize) -> Self {
        Self {
            x: (i % SIDE_LENGTH) as i32,
            y: (i / SIDE_LENGTH) as i32,
        }
    }

    pub fn to_transform(&self, z: f32) -> Transform {
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
pub struct Tile(pub u64);

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
