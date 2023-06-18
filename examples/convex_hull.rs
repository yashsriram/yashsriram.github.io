use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (700., 500.).into(),
            fit_canvas_to_parent: true,
            canvas: Some("#interactive".to_string()),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(init)
    .add_system(game)
    .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor) = window.physical_cursor_position() {
            let cursor = Vec2::new(cursor.x, cursor.y);
            let semi_viewport_axes = Vec2::new(
                window.physical_width() as f32 / 2.,
                window.physical_height() as f32 / 2.,
            );
            let unscaled_click = cursor - semi_viewport_axes;
            let scale_factor = window.scale_factor() as f32;
            let click = unscaled_click / scale_factor;
            info!("click = {:?}", click);
        }
    }
}
