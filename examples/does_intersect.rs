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
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    outputs: Query<Entity, With<SomeOutput>>,
    vertices: Query<&Transform, With<Vertex>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        for entity in &outputs {
            commands.entity(entity).despawn();
        }
        if vertices.iter().len() < 4 {
            return;
        }
        let vs: Vec<_> = vertices
            .iter()
            .take(4)
            .map(|v| v.translation)
            .map(|v| Vec2::new(v.x, v.y))
            .collect();
        let p = vs[0];
        let q = vs[1];
        let r = vs[2];
        let s = vs[3];
        fn sign(p: Vec2, q: Vec2, a: Vec2) -> f32 {
            (a.x - p.x) * (q.y - p.y) - (a.y - p.y) * (q.x - p.x)
        }
        let p_opp_q = sign(r, s, p) * sign(r, s, q) < 0.;
        let r_opp_s = sign(p, q, r) * sign(p, q, s) < 0.;
        let does_intersect = p_opp_q && r_opp_s;
        let line_color = if does_intersect {
            Color::RED
        } else {
            Color::GREEN
        };
        for line_segment in vertices
            .iter()
            .take(4)
            .map(|v| v.translation)
            .collect::<Vec<_>>()
            .chunks(2)
        {
            commands.spawn((
                SomeOutput,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Walk(line_segment).into()).into(),
                    material: materials.add(line_color.into()),
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
