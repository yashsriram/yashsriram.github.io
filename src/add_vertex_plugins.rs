use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{distributions::Uniform, prelude::*};

#[derive(Component)]
pub struct Vertex;

pub struct AddVertexPlugin;

impl Plugin for AddVertexPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_vertex);
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
            let new_vertex_position = Vec3::new(click.x, click.y, 0.);
            commands.spawn((
                Vertex,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(new_vertex_position),
                    ..default()
                },
            ));
        }
    }
}

pub struct AddRandomVerticesPlugin;

impl Plugin for AddRandomVerticesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_with_random_vertices)
            .add_system(add_random_vertices);
    }
}

fn start_with_random_vertices(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let x_range = Uniform::new(-window.width() / 3., window.width() / 3.);
    let y_range = Uniform::new(-window.height() / 3., window.height() / 3.);
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        commands.spawn((
            Vertex,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    x_range.sample(&mut rng),
                    y_range.sample(&mut rng),
                    0.,
                )),
                ..default()
            },
        ));
    }
}

fn add_random_vertices(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F) {
        let window = windows.single();
        let x_range = Uniform::new(-window.width() / 3., window.width() / 3.);
        let y_range = Uniform::new(-window.height() / 3., window.height() / 3.);
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            commands.spawn((
                Vertex,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        x_range.sample(&mut rng),
                        y_range.sample(&mut rng),
                        0.,
                    )),
                    ..default()
                },
            ));
        }
    }
}
