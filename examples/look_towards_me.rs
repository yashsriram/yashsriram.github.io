use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use yashsriram::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
