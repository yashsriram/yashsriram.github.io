pub mod camera_controller;

#[macro_export]
macro_rules! game_3d {
    ($title:literal, $bodies:tt) => {
        use bevy::input::common_conditions::input_just_pressed;
        use bevy::prelude::*;
        use bricks::game::threed::camera_controller::CameraControllerPlugin;

        fn main() {
            let mut app = App::new();
            app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (800.0, 450.0).into(),
                    title: "bricks".into(),
                    canvas: Some("#interactive".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(CameraControllerPlugin)
            .add_systems(Startup, title)
            .add_systems(Startup, init)
            .add_systems(
                Update,
                on_spacebar_press.run_if(input_just_pressed(KeyCode::Space)),
            );
            register_and_draw_bodies!(app, $bodies);
            app.run();
        }

        fn title(mut commands: Commands) {
            // commands.spawn(Text::new($title));
        }
    };
}
