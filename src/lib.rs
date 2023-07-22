use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use rand::{distributions::Uniform, prelude::*};

#[derive(Component)]
pub struct Output;

pub struct TurtleWalk<'a>(pub &'a [Vec3]);

impl From<TurtleWalk<'_>> for Mesh {
    fn from(turtle_walk: TurtleWalk) -> Mesh {
        let vertices: Vec<_> = turtle_walk
            .0
            .iter()
            .map(|point| ([point.x, point.y, point.z], [0., 0., 1.], [0., 0.]))
            .collect();
        let indices = Indices::U32((0..turtle_walk.0.len()).map(|e| e as u32).collect());
        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, p, _)| *p).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

pub fn despawn_on_key_r<S: Component>(
    mut commands: Commands,
    color_materials: Query<Entity, With<S>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for entity in &color_materials {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Vertex;

pub fn add_vertex_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
) -> Result<(), &'static str> {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let cursor = window
            .cursor_position()
            .ok_or("Cursor not found on window")?;
        let semi_viewport_axes = Vec2::new(window.width(), window.height()) / 2.;
        let click = cursor - semi_viewport_axes;
        commands.spawn((
            Vertex,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(click.extend(0.)),
                ..default()
            },
        ));
    }
    Ok(())
}

pub fn start_with_random_vertices(
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
                material: materials.add(Color::WHITE.into()),
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

pub fn add_random_vertices(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F) {
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
