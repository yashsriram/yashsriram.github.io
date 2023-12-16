use crate::*;
impl<'a> From<&CostGuidedTreeSearchResult<'a>> for Mesh {
    fn from(spanning_tree_view: &CostGuidedTreeSearchResult<'a>) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let flattened_tree: Vec<usize> = spanning_tree_view
            .parent_map
            .iter()
            .map(|(&child_idx, &parent_idx)| vec![child_idx, parent_idx.unwrap_or(child_idx)])
            .flatten()
            .collect();
        let positions: Vec<[f32; 3]> = flattened_tree
            .iter()
            .map(|idx| {
                let state = spanning_tree_view.graph.vertices[*idx].state;
                [state.x, state.y, state.z]
            })
            .collect();
        let indices: Vec<u32> = positions
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();
        let indices = Indices::U32(indices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(indices));
        let vertex_colors: Vec<[f32; 4]> = flattened_tree
            .iter()
            .map(|idx| {
                if *idx == spanning_tree_view.start_idx {
                    [1.0, 1.0, 0.0, 1.0]
                } else if *idx == spanning_tree_view.stop_idx {
                    [0.0, 1.0, 0.0, 1.0]
                } else if spanning_tree_view.fringe.contains(idx) {
                    [0.0, 1.0, 1.0, 1.0]
                } else {
                    [1.0, 0.0, 1.0, 0.2]
                }
            })
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        mesh
    }
}
impl From<&Path> for Mesh {
    fn from(path: &Path) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions: Vec<[f32; 3]> = path.vertices.iter().map(|v| [v.x, v.y, v.z]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..path.vertices.len() as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[0.0, 1.0, 0.0, 1.0]; path.vertices.len()];
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}
impl From<&CuboidSpace> for Mesh {
    fn from(space: &CuboidSpace) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let vertex_positions = vec![
            [0.0, 0.0, 0.0],
            [space.size.x, 0.0, 0.0],
            [space.size.x, space.size.y, 0.0],
            [0.0, space.size.y, 0.0],
            [0.0, 0.0, space.size.z],
            [space.size.x, 0.0, space.size.z],
            [space.size.x, space.size.y, space.size.z],
            [0.0, space.size.y, space.size.z],
        ];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        let indices = Indices::U32(vec![
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ]);
        mesh.set_indices(Some(indices));
        let vertex_colors = vec![[1.0, 1.0, 0.0, 0.1]; 8];
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        mesh
    }
}

impl From<&Graph> for Mesh {
    fn from(graph: &Graph) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|Vertex { state, .. }| [state.x, state.y, state.z])
            .collect();
        let indices: Vec<u32> = graph
            .vertices
            .iter()
            .enumerate()
            .map(
                |(
                    v_i,
                    Vertex {
                        state: _,
                        adjacencies: v_adjs,
                    },
                )| {
                    v_adjs
                        .iter()
                        .map(move |&adj| vec![v_i as u32, adj as u32])
                        .flatten()
                },
            )
            .into_iter()
            .flatten()
            .collect();
        let indices = Indices::U32(indices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 1.0, 0.1]; graph.vertices.len()];
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

