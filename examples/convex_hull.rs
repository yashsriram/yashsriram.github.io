use std::cmp::Ordering;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use yashsriram::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (500., 300.).into(),
            canvas: Some("#interactive".to_string()),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(init)
    .add_system(reset)
    .add_system(add_vertex)
    .add_system(convex_hull)
    .run();
}

#[derive(Component)]
struct VertexMarker;

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn reset(
    mut commands: Commands,
    all_elements: Query<Entity, &Handle<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for entity in &all_elements {
            commands.entity(entity).despawn();
        }
    }
}

fn add_vertex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor) = window.cursor_position() {
            let semi_viewport_axes = Vec2::new(window.width() / 2., window.height() / 2.);
            let click = cursor - semi_viewport_axes;
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(click.x, click.y, 0.)),
                    ..default()
                })
                .insert(VertexMarker);
        }
    }
}

fn convex_hull(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, &VertexMarker>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        let mut points: Vec<_> = vertices
            .iter()
            .map(|v| Vec2::new(v.translation.x, v.translation.y))
            .collect();
        if points.len() == 0 {
            return;
        }
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal));
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(Vec3::new(points[0].x, points[0].y, 0.)),
            ..default()
        });
        let mut hull: Vec<Vec2> = vec![];
        // TODO: impl here
        let hull = hull.into_iter().map(|v| Vec3::new(v.x, v.y, 0.)).collect();
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(TurtleWalk(hull).into()).into(),
            material: materials.add(ColorMaterial::from(Color::MAROON)),
            ..default()
        });
        let user_input = vertices.iter().map(|v| v.translation).collect();
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(TurtleWalk(user_input).into()).into(),
            material: materials.add(ColorMaterial::from(Color::CYAN)),
            ..default()
        });
    }
}
