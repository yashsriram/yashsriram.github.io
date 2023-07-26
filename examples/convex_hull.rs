use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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
        .add_startup_system((|| (0.66, 20, true)).pipe(spawn_point_inputs_on_xy))
        .add_system(
            (|keyboard: Res<Input<KeyCode>>| (0.66, 50, keyboard.just_pressed(KeyCode::F)))
                .pipe(spawn_point_inputs_on_xy),
        )
        .add_system(spawn_single_point_input_on_xy)
        .add_startup_system(init)
        .add_system(update)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
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
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(finish),
                ..default()
            },
        ));
        commands.spawn((
            SomeOutput,
            MaterialMesh2dBundle {
                mesh: meshes.add(Walk(&hull).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
        ));
    }
}
