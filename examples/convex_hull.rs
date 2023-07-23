use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::distributions::{Distribution, Uniform};
use std::path::Path;
use yashsriram::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (600., 300.).into(),
                    canvas: Path::new(file!())
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|stem| Some("#".to_string() + stem)),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_system(despawn_on_key_r::<Handle<ColorMaterial>>)
        .add_startup_system(init)
        .add_system(convex_hull)
        .run();
}

fn init(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
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

fn convex_hull(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    vertices: Query<&Transform, With<Vertex>>,
    outputs: Query<Entity, With<Output>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
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
        let finish = vertices
            .iter()
            .map(|v| v.translation)
            .filter(|v| *v != start)
            .reduce(|all_on_left, v| {
                let cross = (all_on_left - start).cross(v - start);
                if cross.z < 0. {
                    v
                } else {
                    all_on_left
                }
            })
            .unwrap_or(Vec3::ZERO);
        let mut hull: Vec<_> = vec![start];
        let mut rem: Vec<_> = vertices.iter().map(|v| v.translation).collect();
        loop {
            let last = *hull.last().unwrap();
            rem = rem.into_iter().filter(|v| *v != last).collect();
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
            hull.push(next);
            if next == finish {
                hull.push(start);
                break;
            }
        }
        commands.spawn((
            Output,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(start),
                ..default()
            },
        ));
        commands.spawn((
            Output,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(finish),
                ..default()
            },
        ));
        commands.spawn((
            Output,
            MaterialMesh2dBundle {
                mesh: meshes.add(Walk(&hull).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
        ));
    }
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
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let cursor = window.cursor_position().unwrap_or(Vec2::ZERO);
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
}
