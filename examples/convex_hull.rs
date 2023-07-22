use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::path::Path;
use yashsriram::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
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
    .add_system(add_vertex_on_click.pipe(error))
    .add_startup_system(start_with_random_vertices)
    .add_system(add_random_vertices)
    .add_system(despawn_on_key_r::<Handle<ColorMaterial>>)
    .add_startup_system(init)
    .add_system(convex_hull)
    .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn convex_hull(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, With<Vertex>>,
    outputs: Query<Entity, With<Output>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::H) {
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
                mesh: meshes.add(TurtleWalk(&hull).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
        ));
    }
}
