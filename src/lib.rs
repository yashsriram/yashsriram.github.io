use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct SomeOutput;

#[derive(Component)]
pub struct PointInput;

pub struct Walk<'a>(pub &'a [Vec3]);

impl From<Walk<'_>> for Mesh {
    fn from(turtle_walk: Walk) -> Mesh {
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
    entities_with_s: Query<Entity, With<S>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        for entity in &entities_with_s {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_point_inputs_on_xy(
    In((window_scale, num_samples, spawn)): In<(f32, usize, bool)>,
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if spawn {
        let window = windows.single();
        let x_range = Uniform::new(
            -window.width() * 0.5 * window_scale,
            window.width() * 0.5 * window_scale,
        );
        let y_range = Uniform::new(
            -window.height() * 0.5 * window_scale,
            window.height() * 0.5 * window_scale,
        );
        let mut rng = rand::thread_rng();
        for _ in 0..num_samples {
            commands.spawn((
                PointInput,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(
                        [x_range.sample(&mut rng), y_range.sample(&mut rng), 0.].into(),
                    ),
                    ..default()
                },
            ));
        }
    }
}

pub fn spawn_single_point_input_on_xy(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let cursor = window.cursor_position().unwrap_or(Vec2::ZERO);
        let semi_viewport_axes = Vec2::new(window.width(), window.height()) / 2.;
        let click = cursor - semi_viewport_axes;
        commands.spawn((
            PointInput,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(click.extend(0.)),
                ..default()
            },
        ));
    }
}
