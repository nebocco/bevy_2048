use bevy::prelude::*;

mod components;
mod systems;

use self::{components::*, systems::*};

const WINDOW_SIZE: f32 = 500.;
const TILE_SIZE: f32 = 60.;
const SIDE_LENGTH: usize = 4;

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
