use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::path::Path;
use yashsriram::*;

fn main() {
    let mut app = App::new();
    let canvas_id = Path::new(file!())
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| Some("#".to_string() + stem));
    let window_size = (600., 300.);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: window_size.into(),
            canvas: canvas_id,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(AddVertexPlugin)
    .add_plugin(DespawnOnKeyRPlugin::<Handle<ColorMaterial>>::default())
    .add_startup_system(init)
    .add_system(does_intersect)
    .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn does_intersect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    outputs: Query<Entity, With<Output>>,
    vertices: Query<&Transform, With<Vertex>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
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
                Output,
                MaterialMesh2dBundle {
                    mesh: meshes.add(TurtleWalk(line_segment).into()).into(),
                    material: materials.add(line_color.into()),
                    ..default()
                },
            ));
        }
    }
}
