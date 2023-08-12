use bevy::render::mesh::Mesh;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};
use bevy_flycam::PlayerPlugin;

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "ebf24026-f0c7-4e86-8a4a-96a40101d1b5"]
pub struct SimpleMaterial {}

impl Material for SimpleMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Debug, Component)]
pub struct DiffDrive {
    pub radius: f32,
}

impl DiffDrive {
    const POLYGON_SIZE: usize = 18;

    pub fn update(transform: &mut Transform, v: f32, w: f32, dt: f32) {
        let (axis, angle) = transform.rotation.to_axis_angle();
        let orient_in_rad = axis.z.signum() * angle;
        transform.translation.x += v * orient_in_rad.cos() * dt;
        transform.translation.y += v * orient_in_rad.sin() * dt;
        transform.rotation *= Quat::from_rotation_z(w * dt);
    }
}

impl From<&DiffDrive> for Mesh {
    fn from(diff_drive: &DiffDrive) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, {
            let mut positions: Vec<[f32; 3]> = (0..=DiffDrive::POLYGON_SIZE)
                .map(|i| 2.0 * std::f32::consts::PI / DiffDrive::POLYGON_SIZE as f32 * i as f32)
                .map(|theta| {
                    [
                        diff_drive.radius * theta.cos(),
                        diff_drive.radius * theta.sin(),
                        0.0,
                    ]
                })
                .collect();
            positions.push([0.0, 0.0, 0.0]);
            positions
        });
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 1.0, 1.0]; DiffDrive::POLYGON_SIZE + 2],
        );
        mesh.set_indices(Some(Indices::U32(
            (0..=(DiffDrive::POLYGON_SIZE + 1))
                .map(|i| i as u32)
                .collect(),
        )));
        mesh
    }
}

#[derive(Component)]
pub struct DrawPath {
    pub len: usize,
}

impl DrawPath {
    pub fn add_point(&self, mesh: &mut Mesh, point: [f32; 3]) {
        if let Some(VertexAttributeValues::Float32x3(ref mut vec)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            vec.push(point);
        }
        if let Some(VertexAttributeValues::Float32x4(ref mut vec)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
        {
            vec.push([1.0, 1.0, 1.0, 0.2]);
        }
        if let Some(Indices::U32(ref mut vec)) = mesh.indices_mut() {
            vec.push(vec.len() as u32);
        }
    }
}

impl From<&DrawPath> for Mesh {
    fn from(_: &DrawPath) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 1.0, 0.1]]);
        mesh.set_indices(Some(Indices::U32(vec![0])));
        mesh
    }
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
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(init)
        .add_system(look_towards_me)
        .run();
}

#[derive(Copy, Clone, Resource)]
struct TargetAngleRad(f32);

fn init(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SimpleMaterial>>,
) {
    let diff_drive = DiffDrive { radius: 1.0 };
    commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_assets.add(Mesh::from(&diff_drive)),
            material: materials.add(SimpleMaterial {}),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(diff_drive);

    let target_angle_rad = TargetAngleRad(4.0);
    commands.insert_resource(target_angle_rad);
    let path = DrawPath { len: 200 };
    let mut mesh = Mesh::from(&path);
    path.add_point(
        &mut mesh,
        [
            10.0 * target_angle_rad.0.cos(),
            10.0 * target_angle_rad.0.sin(),
            0.0,
        ],
    );
    commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_assets.add(mesh),
            material: materials.add(SimpleMaterial {}),
            ..Default::default()
        })
        .insert(path);
}

fn look_towards_me(
    mut diff_drive_query: Query<(&DiffDrive, &mut Transform)>,
    target_angle_rad: Res<TargetAngleRad>,
) {
    let (_, mut diff_drive_transform) = diff_drive_query.single_mut();
    let (axis, angle) = diff_drive_transform.rotation.to_axis_angle();
    let orient_in_rad = axis.z.signum() * angle;
    let unit_vec = Vec3::new(orient_in_rad.cos(), orient_in_rad.sin(), 0.0);
    let target_unit_vec = Vec3::new(target_angle_rad.0.cos(), target_angle_rad.0.sin(), 0.0);
    let cross_product = unit_vec.cross(target_unit_vec);
    let w = cross_product.length().min(0.1) * cross_product.z.signum();
    DiffDrive::update(&mut *diff_drive_transform, 0.0, w, 0.1);
}
