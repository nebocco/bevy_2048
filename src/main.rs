use bevy::prelude::*;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256StarStar};

mod components;
mod systems;

use self::{components::*, systems::*};

const WINDOW_SIZE: f32 = 500.;
const TILE_SIZE: f32 = 60.;
const SIDE_LENGTH: usize = 4;
const TILE_NUM: usize = SIDE_LENGTH * SIDE_LENGTH;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Move,
    Spawn,
    GameOver,
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
        .insert_resource(Xoshiro256StarStar::from_entropy())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(GameState::Move)
        .add_event::<MoveEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Move).with_system(check_game_over))
        .add_system_set(
            SystemSet::on_update(GameState::Move)
                .with_system(send_move_event)
                .with_system(move_tiles_system),
        )
        .add_system_set(SystemSet::on_update(GameState::Spawn).with_system(return_to_move_state))
        .add_system_set(SystemSet::on_exit(GameState::Spawn).with_system(create_random_tile))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(end_game))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
