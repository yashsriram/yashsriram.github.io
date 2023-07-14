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
struct Vertex;

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
                .insert(Vertex);
        }
    }
}

fn convex_hull(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<&Transform, With<Vertex>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        // let user_input = vertices.iter().map(|v| v.translation).collect();
        // commands.spawn(MaterialMesh2dBundle {
        //     mesh: meshes.add(TurtleWalk(user_input).into()).into(),
        //     material: materials.add(ColorMaterial::from(Color::CYAN)),
        //     ..default()
        // });
        let seed = vertices
            .iter()
            .map(|v| v.translation)
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .unwrap_or(Vec3::ZERO);
        let mut hull = vec![seed];
        for _ in 0..50 {
            let seed2 = vertices
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
            hull.push(seed2);
        }
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(seed),
            ..default()
        });
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(TurtleWalk(hull).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        });
    }
}
