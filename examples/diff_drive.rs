use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use rand::{thread_rng, Rng};
use yashsriram::*;

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
