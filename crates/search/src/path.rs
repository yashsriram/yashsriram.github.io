use super::graph::*;
use super::search::*;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Path {
    pub vertices: Vec<Vec3>,
}

impl Path {
    pub fn generate_for<'a>(&mut self, graph: &'a Graph, ts: &CostGuidedTreeSearchResult) {
        let vertices = match ts.path_to_stop(graph) {
            None => vec![],
            Some(path) => path
                .into_iter()
                .map(|idx| graph.vertices[idx].pos)
                .collect(),
        };
        self.vertices = vertices;
    }
}
