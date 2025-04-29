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
pub struct LineSegment;
#[derive(Component)]
pub struct InputPoint;

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut commands: Commands,
    inputs: Query<Entity, With<InputPoint>>,
    outputs: Query<Entity, With<LineSegment>>,
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
        let points: [Vec2; 4] = core::array::from_fn(|_| {
            Vec2::new(
                window.width() * (rng.gen::<f32>() - 0.5),
                window.height() * (rng.gen::<f32>() - 0.5),
            )
        });
        let [p, q, r, s] = points;
        fn sign(p: Vec2, q: Vec2, a: Vec2) -> f32 {
            (a.x - p.x) * (q.y - p.y) - (a.y - p.y) * (q.x - p.x)
        }
        let p_opp_q = sign(r, s, p) * sign(r, s, q) < 0.;
        let r_opp_s = sign(p, q, r) * sign(p, q, s) < 0.;
        let does_intersect = p_opp_q && r_opp_s;
        for point in points {
            commands.spawn((
                InputPoint,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(point.extend(0.)),
                    ..default()
                },
            ));
        }
        for line_segment in points.map(|v| v.extend(0.)).chunks(2) {
            commands.spawn((
                LineSegment,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Walk(line_segment).into()).into(),
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
