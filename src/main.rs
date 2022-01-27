use bevy::prelude::*;

const WINDOW_SIZE: f32 = 500.;

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
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
