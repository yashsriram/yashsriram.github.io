use super::spaces::*;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

#[derive(Default)]
pub struct Vertex {
    pub(crate) state: Vec3,
    pub(crate) adjacencies: HashSet<usize>,
}

#[derive(Resource, Default)]
pub struct Graph {
    pub(crate) vertices: Vec<Vertex>,
}

impl Graph {
    pub fn sample(&mut self, space: &CuboidWithHoldSpace, num_samples: usize, edge_len: f32) {
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
            .map(|(state, adjacencies)| Vertex { state, adjacencies })
            .collect();
    }

    pub fn add<const N: usize>(&mut self, states: [Vec3; N], edge_len: f32) -> [usize; N] {
        let prev_graph_size = self.vertices.len();
        for state in IntoIterator::into_iter(states) {
            self.vertices.push(Vertex {
                state,
                adjacencies: HashSet::new(),
            });
        }
        for i in (prev_graph_size..self.vertices.len()).rev() {
            for j in 0..(i - 1) {
                if (self.vertices[i].state - self.vertices[j].state).length() <= edge_len {
                    self.vertices[i].adjacencies.insert(j);
                    self.vertices[j].adjacencies.insert(i);
                }
            }
        }
        let mut idxes = [0; N];
        for (i, idx) in (prev_graph_size..self.vertices.len()).enumerate() {
            idxes[i] = idx;
        }
        idxes
    }
}
