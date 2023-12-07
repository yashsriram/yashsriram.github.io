use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{sprite::MaterialMesh2dBundle};
use rand::prelude::*;

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

#[derive(Component)]
pub struct Ray;
#[derive(Component)]
pub struct InputPoint;

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut commands: Commands,
    inputs: Query<Entity, With<InputPoint>>,
    outputs: Query<Entity, With<Ray>>,
    keyboard: Res<Input<KeyCode>>,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for entity in &inputs {
            commands.entity(entity).despawn();
        }
        for entity in &outputs {
            commands.entity(entity).despawn();
        }
        let window = windows.single();
        let mut rng = rand::thread_rng();
        let points: [Vec3; 4] = core::array::from_fn(|_| {
            Vec3::new(
                window.width() * (rng.gen::<f32>() - 0.5),
                window.height() * (rng.gen::<f32>() - 0.5),
                0.,
            )
        });
        let [p1, p2, q1, q2] = points;
        let r1 = (p2 - p1).normalize();
        let r2 = (q2 - q1).normalize();
        let base = (q1 - p1).normalize();
        let ray1_towards_ray2 = r1.dot(base);
        let ray2_towards_ray1 = r2.dot(-base);
        let towards_each_other = ray1_towards_ray2 + ray2_towards_ray1 > 0.;
        let same_side_of_base = r1.cross(base).z * r2.cross(-base).z < 0.;
        let does_intersect = towards_each_other && same_side_of_base;
        for ray in points.chunks(2) {
            commands.spawn((
                InputPoint,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(ray[0]),
                    ..default()
                },
            ));
            commands.spawn((
                InputPoint,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(8., 3).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::default()
                        .with_rotation(Quat::from_rotation_arc(
                            Vec3::Y,
                            (ray[1] - ray[0]).try_normalize().unwrap_or(Vec3::Y),
                        ))
                        .with_translation(ray[1]),
                    ..default()
                },
            ));
        }
        for ray in points.chunks(2) {
            commands.spawn((
                Ray,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Walk(ray).into()).into(),
                    material: materials.add(
                        if does_intersect {
                            Color::RED
                        } else {
                            Color::GREEN
                        }
                        .into(),
                    ),
                    ..default()
                },
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (600., 300.).into(),
                canvas: Some("#interactive".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(init)
        .add_system(update)
        .run();
}
