mod camera_controller;
pub use camera_controller::CameraController;
pub use camera_controller::CameraControllerPlugin;

#[macro_export]
macro_rules! register_and_draw_bodies {
    (
        $app:ident,
        {
            $resource:ident -> $drawing:ident,
            $($next_resource:ident -> $next_drawing:ident,)*
        } ) => {
        $app.init_resource::<$resource>();
        $app.add_systems(
            Update,
            $drawing,
        );
        register_and_draw_bodies!($app, { $($next_resource -> $next_drawing,)* })
    };
    ($app:ident, {}) => {};
}

#[macro_export]
macro_rules! simple_vis {
    ($title:literal, $bodies:tt) => {
        use bevy::input::common_conditions::input_just_pressed;
        use bevy::prelude::*;
        use rand::prelude::*;

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
            .add_systems(Update, mouse_click_on_screen.pipe(on_mouse_click))
            .add_systems(
                Update,
                on_spacebar_press.run_if(input_just_pressed(KeyCode::Space)),
            );
            register_and_draw_bodies!(app, $bodies);
            app.run();
        }

        fn title(mut commands: Commands) {
            commands.spawn(Text::new($title));
        }

        fn mouse_click_on_screen(
            camera_query: Single<(&Camera, &GlobalTransform)>,
            window: Single<&Window>,
            mouse: Res<ButtonInput<MouseButton>>,
        ) -> Result<Vec2, ()> {
            if mouse.just_pressed(MouseButton::Left) {
                let (camera, camera_transform) = *camera_query;
                let Some(cursor_position) = window.cursor_position() else {
                    return Err(());
                };
                let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position)
                else {
                    return Err(());
                };
                return Ok(point);
            }
            return Err(())
        }
    };
}
