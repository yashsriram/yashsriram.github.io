use bevy_flycam::PlayerPlugin;
use rand::{thread_rng, Rng};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::Mesh;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<SimpleMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(PlayerPlugin)
        .add_startup_system(
            |mut commands: Commands,
             mut mesh_assets: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<SimpleMaterial>>| {
                let diff_drive = DiffDrive { radius: 1.0 };
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&diff_drive)),
                        material: materials.add(SimpleMaterial {}),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(diff_drive);
                let path = DrawPath { len: 200 };
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&path)),
                        material: materials.add(SimpleMaterial {}),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(path);
            },
        )
        .add_system(
            |mut mesh_assets: ResMut<Assets<Mesh>>,
             mut diff_drive_query: Query<(&DiffDrive, &mut Transform)>,
             mut path_query: Query<(&DrawPath, &Handle<Mesh>)>| {
                let mut rng = thread_rng();
                let (_, mut diff_drive_transform) = diff_drive_query.single_mut();
                DiffDrive::update(
                    &mut *diff_drive_transform,
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    0.1,
                );

                let (path, path_mesh_handle) = path_query.single_mut();
                let mesh = mesh_assets.get_mut(path_mesh_handle).unwrap();
                path.add_point(
                    mesh,
                    [
                        diff_drive_transform.translation.x,
                        diff_drive_transform.translation.y,
                        diff_drive_transform.translation.z,
                    ],
                );
            },
        )
        .run();
}
