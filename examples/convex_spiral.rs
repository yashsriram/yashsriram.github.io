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
    .add_system(convex_spiral)
    .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn convex_spiral(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, With<Vertex>>,
    outputs: Query<Entity, With<Output>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::S) {
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
                mesh: meshes.add(TurtleWalk(&spiral).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
        ));
    }
}
