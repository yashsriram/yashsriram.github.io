use bevy::render::mesh::Mesh;

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};
use bevy::{
    prelude::{shape::Cube},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_flycam::PlayerPlugin;

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "ebf24026-f0c7-4e86-8a4a-96a40101d1b5"]
pub struct SimpleMaterial {}

impl Material for SimpleMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Component)]
struct ConfigSpacePoint;
#[derive(Component)]
struct FixedMarker;
#[derive(Component)]
struct RotatedMarker;

#[derive(Resource)]
struct GameState {
    pub r: f32,
    pub theta: f32,
    pub d: f32,
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
        .add_plugin(MaterialPlugin::<SimpleMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(PlayerPlugin)
        .add_startup_system(
            |mut commands: Commands,
             mut mesh_assets: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<SimpleMaterial>>| {
                commands.insert_resource(GameState {
                    r: 1.0,
                    theta: 0.0,
                    d: 0.0,
                });
                let axes = Axes { len: 10.0 };
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&axes)),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    })
                    .insert(axes);
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(ColoredMesh::of(&Thing))),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    })
                    .insert(FixedMarker);
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&Thing)),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    })
                    .insert(RotatedMarker);
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(ColoredMesh::of(Cube { size: 0.1 }))),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    })
                    .insert(ConfigSpacePoint);
            },
        )
        .add_system(
            |mut game_state: ResMut<GameState>, keyboard_input: Res<Input<KeyCode>>| {
                if keyboard_input.pressed(KeyCode::Right) && game_state.r + 0.01 <= 1.0 {
                    game_state.r += 0.01
                }
                if keyboard_input.pressed(KeyCode::Left) && game_state.r - 0.01 >= 0.0 {
                    game_state.r -= 0.01
                }
                if keyboard_input.pressed(KeyCode::Up) {
                    game_state.theta += 0.01
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    game_state.theta -= 0.01
                }
                if keyboard_input.pressed(KeyCode::PageUp) {
                    game_state.d += 0.01
                }
                if keyboard_input.pressed(KeyCode::PageDown) {
                    game_state.d -= 0.01
                }
            },
        )
        .add_system(
            |game_state: Res<GameState>, mut q1: Query<(&mut RotatedMarker, &mut Transform)>| {
                let (_, mut thing_transform) = q1.single_mut();
                let r = game_state.r;
                let theta = game_state.theta;
                let d = game_state.d;
                let x_hat = r * theta.cos();
                let z_hat = r * theta.sin();
                let y_hat = (1.0 - x_hat * x_hat - z_hat * z_hat).sqrt();
                let angle = d;
                thing_transform.rotation =
                    Quat::from_axis_angle(Vec3::new(x_hat, y_hat, z_hat).normalize(), angle);
            },
        )
        .add_system(
            |game_state: Res<GameState>, mut q2: Query<(&ConfigSpacePoint, &mut Transform)>| {
                let (_, mut config_space_point_transform) = q2.single_mut();
                let r = game_state.r;
                let theta = game_state.theta;
                let d = game_state.d;
                let x_hat = r * theta.cos();
                let z_hat = r * theta.sin();
                let _y_hat = (1.0 - x_hat * x_hat - z_hat * z_hat).sqrt();
                let angle = d;
                config_space_point_transform.translation = Vec3::new(x_hat, angle, z_hat);
            },
        )
        .run();
}

#[derive(Debug, Component)]
pub struct Axes {
    pub len: f32,
}

impl From<&Axes> for Mesh {
    fn from(axes: &Axes) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-axes.len, 0.0, 0.0],
                [axes.len, 0.0, 0.0],
                [0.0, -axes.len, 0.0],
                [0.0, axes.len, 0.0],
                [0.0, 0.0, -axes.len],
                [0.0, 0.0, axes.len],
            ],
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![
                [1.0, 0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
            ],
        );
        mesh.set_indices(Some(Indices::U32((0..=6).map(|i| i as u32).collect())));
        mesh
    }
}

pub struct Thing;

impl From<&Thing> for Mesh {
    fn from(_: &Thing) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions = vec![
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [3.0, 5.0, 0.0],
            [0.0, 0.0, 0.0],
        ];
        let count = positions.len();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0, 0.0, 1.0, 1.0]; count]);
        mesh.set_indices(Some(Indices::U32((0..count).map(|i| i as u32).collect())));
        mesh
    }
}

pub struct ColoredMesh<S: Into<Mesh>> {
    pub raw_mesh: S,
}

impl<S: Into<Mesh>> ColoredMesh<S> {
    pub fn of(raw_mesh: S) -> Self {
        Self { raw_mesh }
    }
}

impl<S: Into<Mesh>> From<ColoredMesh<S>> for Mesh {
    fn from(mesh: ColoredMesh<S>) -> Self {
        let mut raw_mesh: Mesh = mesh.raw_mesh.into();
        raw_mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 0.0, 1.0]; raw_mesh.count_vertices()],
        );
        raw_mesh
    }
}
