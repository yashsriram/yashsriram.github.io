use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use yashsriram::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<SimpleMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(PlayerPlugin)
        .add_startup_system(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<SimpleMaterial>>| {
                let space = CuboidSpace {
                    size: Vec3::new(12.0, 10.0, 5.0),
                };
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(&space)).into(),
                    material: materials.add(SimpleMaterial {}),
                    ..Default::default()
                });
                let mut prm = PRM::with_num_samples(&space, 2000, 1.0);
                let [a, b] = prm.add([Vec3::new(0.3, 0.7, 0.5), Vec3::new(9.5, 7.3, 4.0)], 1.0);
                let searches = [
                    DFS::try_on(&prm.graph, a, b),
                    BFS::try_on(&prm.graph, a, b),
                    UCS::try_on(&prm.graph, a, b),
                    AStar::try_on(&prm.graph, a, b),
                    WeightableAStar::<11, 10>::try_on(&prm.graph, a, b),
                    AStarWeighted2::try_on(&prm.graph, a, b),
                ];
                for (i, search) in searches.into_iter().enumerate() {
                    commands.spawn(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(&Path::from(&search))).into(),
                        transform: Transform::from_xyz((i + 1) as f32 * 14.0, 0.0, 0.0),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    });
                    commands.spawn(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(&search)).into(),
                        transform: Transform::from_xyz((i + 1) as f32 * 14.0, 0.0, 0.0),
                        material: materials.add(SimpleMaterial {}),
                        ..Default::default()
                    });
                }
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(&prm.graph)).into(),
                    material: materials.add(SimpleMaterial {}),
                    ..Default::default()
                });
            },
        )
        .run();
}
