use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

#[derive(Component)]
pub struct Output;

#[derive(Component)]
pub struct Vertex;

pub struct Walk<'a>(pub &'a [Vec3]);

impl From<Walk<'_>> for Mesh {
    fn from(turtle_walk: Walk) -> Mesh {
        let vertices: Vec<_> = turtle_walk
            .0
            .iter()
            .map(|point| ([point.x, point.y, point.z], [0., 0., 1.], [0., 0.]))
            .collect();
        let indices = Indices::U32((0..turtle_walk.0.len()).map(|e| e as u32).collect());
        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, p, _)| *p).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

pub fn despawn_on_key_r<S: Component>(
    mut commands: Commands,
    color_materials: Query<Entity, With<S>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for entity in &color_materials {
            commands.entity(entity).despawn();
        }
    }
}
