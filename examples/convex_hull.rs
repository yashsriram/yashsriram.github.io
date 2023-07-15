use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use yashsriram::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (600., 600.).into(),
            canvas: Some("#interactive".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
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
struct Vertex;
#[derive(Component)]
struct Output;

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn reset(
    mut commands: Commands,
    color_materials: Query<Entity, With<Handle<ColorMaterial>>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for entity in &color_materials {
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
                .insert(Vertex);
        }
    }
}

fn convex_hull(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, With<Vertex>>,
    outputs: Query<Entity, With<Output>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
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
        let mut hull = vec![start];
        loop {
            let next = vertices
                .iter()
                .map(|v| v.translation)
                .filter(|v| !hull.contains(v))
                .reduce(|all_on_right, v| {
                    let last = *hull.last().unwrap();
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
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(start),
                ..default()
            })
            .insert(Output);
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(finish),
                ..default()
            })
            .insert(Output);
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(TurtleWalk(hull).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            })
            .insert(Output);
    }
}
