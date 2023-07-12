use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct Scribble {
    pub points: Vec<Vec3>,
}

impl Default for Scribble {
    fn default() -> Self {
        Scribble {
            points: vec![
                Vec3::new(0., 0., 0.),
                Vec3::new(10., 0., 0.),
                Vec3::new(0., 10., 0.),
                Vec3::new(0., 0., 0.),
            ],
        }
    }
}

impl From<Scribble> for Mesh {
    fn from(path: Scribble) -> Mesh {
        let vertices: Vec<_> = path
            .points
            .iter()
            .map(|point| ([point.x, point.y, point.z], [0., 0., 1.], [0., 0.]))
            .collect();
        let indices = Indices::U32((0..path.points.len()).map(|e| e as u32).collect());
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
