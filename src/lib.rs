#[macro_export]
macro_rules! register_resources {
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
        register_resources!($app, { $($next_resource($next_gizmos, $next_resource_name) => $next_drawing,)* })
    };
    ($app:ident, {}) => {};
}

#[macro_export]
macro_rules! visualize {
    ($bodies:tt) => {
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
            .add_systems(Update, (add_point_at_mouse_click))
            .add_systems(Update, (algo.run_if(input_just_pressed(KeyCode::Space))));
            register_resources!(app, $bodies);
            app.run();
        }

        fn init(mut commands: Commands) {
            commands.spawn(Camera2d::default());
        }
    };
}
