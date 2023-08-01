use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;
use yashsriram::*;

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
