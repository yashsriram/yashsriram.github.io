use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (400., 200.).into(),
            canvas: Some("#interactive".to_string()),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(init)
    .add_system(reset)
    .add_system(add_vertex)
    .add_system(convex_hull)
    .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn reset(
    mut commands: Commands,
    query: Query<Entity, &Handle<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        info!("Reset");
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

fn add_vertex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor) = window.cursor_position() {
            let semi_viewport_axes = Vec2::new(window.width() / 2., window.height() / 2.);
            let click = cursor - semi_viewport_axes;
            info!("click = {:?}", click);
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(click.x, click.y, 0.)),
                ..default()
            });
        }
    }
}

fn convex_hull(
    mut commands: Commands,
    query: Query<Entity, &Handle<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        info!("Convex hull");
    }
}
