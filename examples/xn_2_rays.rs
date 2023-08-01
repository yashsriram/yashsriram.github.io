use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;
use yashsriram::*;

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
