use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;
use yashsriram::*;

#[derive(Component)]
pub struct SomeOutput;
#[derive(Component)]
pub struct PointInput;

fn init(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let window = windows.single();
    let mut rng = rand::thread_rng();
    let points: [Vec2; 4] = core::array::from_fn(|_| {
        Vec2::new(
            window.width() * (rng.gen::<f32>() - 0.5),
            window.height() * (rng.gen::<f32>() - 0.5),
        )
    });
    for point in points {
        commands.spawn((
            PointInput,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(point.extend(0.)),
                ..default()
            },
        ));
    }
}

fn algo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, With<PointInput>>,
    outputs: Query<Entity, With<SomeOutput>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        for entity in &outputs {
            commands.entity(entity).despawn();
        }
        let start = vertices
            .iter()
            .map(|v| v.translation)
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .unwrap_or(Vec3::ZERO);
        let mut spiral: Vec<_> = vec![start];
        let mut rem: Vec<_> = vertices.iter().map(|v| v.translation).collect();
        loop {
            let last = *spiral.last().unwrap();
            rem = rem.into_iter().filter(|v| *v != last).collect();
            if rem.len() == 0 {
                break;
            }
            let next = rem
                .clone()
                .into_iter()
                .reduce(|all_on_right, v| {
                    let cross = (all_on_right - last).cross(v - last);
                    if cross.z > 0. {
                        v
                    } else {
                        all_on_right
                    }
                })
                .unwrap_or(Vec3::ZERO);
            spiral.push(next);
        }
        commands.spawn((
            SomeOutput,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(start),
                ..default()
            },
        ));
        commands.spawn((
            SomeOutput,
            MaterialMesh2dBundle {
                mesh: meshes.add(Walk(&spiral).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
        ));
    }
}

fn clear(
    mut commands: Commands,
    color_materials: Query<Entity, With<Handle<ColorMaterial>>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        for entity in &color_materials {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_few(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F) {
        let window = windows.single();
        let mut rng = rand::thread_rng();
        let points: [Vec2; 20] = core::array::from_fn(|_| {
            Vec2::new(
                window.width() * (rng.gen::<f32>() - 0.5),
                window.height() * (rng.gen::<f32>() - 0.5),
            )
        });
        for point in points {
            commands.spawn((
                PointInput,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(point.extend(0.)),
                    ..default()
                },
            ));
        }
    }
}

pub fn spawn_one(
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
        .add_system(clear)
        .add_system(algo)
        .add_system(spawn_few)
        .add_system(spawn_one)
        .run();
}
