use super::spaces::*;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

#[derive(Default)]
pub struct Vertex {
    pub pos: Vec3,
    pub adjacencies: HashSet<usize>,
}

#[derive(Resource, Default)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
}

impl Graph {
    pub fn generate_samples(
        &mut self,
        space: &CuboidWithHoldSpace,
        num_samples: usize,
        edge_len: f32,
    ) {
        let mut rng = thread_rng();
        let state_samples: Vec<Vec3> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(x, y, z): (f32, f32, f32)| {
                Vec3::new(x * space.size.x, y * space.size.y, z * space.size.z) - space.size / 2.0
            })
            .filter(|point| point.length() > space.hole_radius)
            .collect();
        let mut adjacencies = vec![HashSet::new(); state_samples.len()];
        for i in 0..(state_samples.len() - 1) {
            let s1 = state_samples[i];
            for j in (i + 1)..state_samples.len() {
                let s2 = state_samples[j];
                if (s1 - s2).length() <= edge_len {
                    adjacencies[i].insert(j);
                    adjacencies[j].insert(i);
                }
            }
        }
        self.vertices = state_samples
            .into_iter()
            .zip(adjacencies.into_iter())
            .map(|(state, adjacencies)| Vertex {
                pos: state,
                adjacencies,
            })
            .collect();
    }

    pub fn choose_random_vertex_idx(&self) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..self.vertices.len())
    }
}
