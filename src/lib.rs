#[macro_export]
macro_rules! register_and_draw_bodies {
    (
        $app:ident,
        {
            $resource:ident($gizmos:ident, $resource_name:ident) => $drawing:block,
            $($next_resource:ident($next_gizmos:ident, $next_resource_name:ident) => $next_drawing:block,)*
        } ) => {
        $app.init_resource::<$resource>();
        $app.add_systems(
            Update,
            (
                |mut $gizmos: Gizmos, $resource_name: Res<$resource>| $drawing
            ),
        );
        register_and_draw_bodies!($app, { $($next_resource($next_gizmos, $next_resource_name) => $next_drawing,)* })
    };
    ($app:ident, {}) => {};
}

#[macro_export]
macro_rules! vis_2d {
    ($title:literal, $bodies:tt) => {
        use bevy::input::common_conditions::input_just_pressed;
        use bevy::prelude::*;

        fn main() {
            let mut app = App::new();
            app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (500., 400.).into(),
                    canvas: Some("#interactive".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, init)
            .add_systems(Update, (mouse_click_on_screen.pipe(on_mouse_click)))
            .add_systems(
                Update,
                (on_spacebar_press.run_if(input_just_pressed(KeyCode::Space))),
            );
            register_and_draw_bodies!(app, $bodies);
            app.run();
        }

        fn init(mut commands: Commands) {
            commands.spawn(Camera2d::default());
            commands.spawn(Text::new($title));
        }

        fn mouse_click_on_screen(
            camera_query: Single<(&Camera, &GlobalTransform)>,
            windows: Single<&Window>,
            mouse: Res<ButtonInput<MouseButton>>,
        ) -> Result<Vec2, ()> {
            if mouse.just_pressed(MouseButton::Left) {
                let (camera, camera_transform) = *camera_query;
                let Some(cursor_position) = windows.cursor_position() else {
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
