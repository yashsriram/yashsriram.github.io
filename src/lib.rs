use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct SomeOutput;

#[derive(Component)]
pub struct PointInput;

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
    entities_with_s: Query<Entity, With<S>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        for entity in &entities_with_s {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_point_inputs_on_xy(
    In((window_scale, num_samples, spawn)): In<(f32, usize, bool)>,
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if spawn {
        let window = windows.single();
        let x_range = Uniform::new(
            -window.width() * 0.5 * window_scale,
            window.width() * 0.5 * window_scale,
        );
        let y_range = Uniform::new(
            -window.height() * 0.5 * window_scale,
            window.height() * 0.5 * window_scale,
        );
        let mut rng = rand::thread_rng();
        for _ in 0..num_samples {
            commands.spawn((
                PointInput,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(
                        [x_range.sample(&mut rng), y_range.sample(&mut rng), 0.].into(),
                    ),
                    ..default()
                },
            ));
        }
    }
}

pub fn spawn_single_point_input_on_xy(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let cursor = window.cursor_position().unwrap_or(Vec2::ZERO);
        let semi_viewport_axes = Vec2::new(window.width(), window.height()) / 2.;
        let click = cursor - semi_viewport_axes;
        commands.spawn((
            PointInput,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(click.extend(0.)),
                ..default()
            },
        ));
    }
}

use rand::distributions::Standard;
use rand::{thread_rng, Rng};

pub struct PRM {
    pub graph: Graph,
}

impl PRM {
    pub fn with_num_samples(state_space: &CuboidSpace, num_samples: usize, edge_len: f32) -> Self {
        let mut rng = thread_rng();
        let state_samples: Vec<Vec3> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(x, y, z): (f32, f32, f32)| {
                Vec3::new(
                    x * state_space.size.x,
                    y * state_space.size.y,
                    z * state_space.size.z,
                )
            })
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
        Self {
            graph: Graph {
                vertices: state_samples
                    .into_iter()
                    .zip(adjacencies.into_iter())
                    .map(|(state, adjacencies)| Vertex { state, adjacencies })
                    .collect(),
            },
        }
    }

    pub fn add<const N: usize>(&mut self, states: [Vec3; N], edge_len: f32) -> [usize; N] {
        let prev_graph_size = self.graph.vertices.len();
        for state in IntoIterator::into_iter(states) {
            self.graph.vertices.push(Vertex {
                state,
                adjacencies: HashSet::new(),
            });
        }
        for i in (prev_graph_size..self.graph.vertices.len()).rev() {
            for j in 0..(i - 1) {
                if (self.graph.vertices[i].state - self.graph.vertices[j].state).length()
                    <= edge_len
                {
                    self.graph.vertices[i].adjacencies.insert(j);
                    self.graph.vertices[j].adjacencies.insert(i);
                }
            }
        }
        let mut idxes = [0; N];
        for (i, idx) in (prev_graph_size..self.graph.vertices.len()).enumerate() {
            idxes[i] = idx;
        }
        idxes
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub(crate) state: Vec3,
    pub(crate) adjacencies: HashSet<usize>,
}

#[derive(Debug)]
pub struct Graph {
    pub(crate) vertices: Vec<Vertex>,
}

impl Graph {
    pub fn num_edges(&self) -> usize {
        self.vertices
            .iter()
            .map(
                |Vertex {
                     state: _,
                     adjacencies,
                 }| adjacencies.len(),
            )
            .sum()
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

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "ebf24026-f0c7-4e86-8a4a-96a40101d1b5"]
pub struct SimpleMaterial {}

impl Material for SimpleMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

use bevy::render::mesh::VertexAttributeValues;

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

use ordered_float::OrderedFloat;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug)]
pub struct CostGuidedTreeSearchResult<'a> {
    pub(crate) graph: &'a Graph,
    start_idx: usize,
    stop_idx: usize,
    reached: bool,
    pub(crate) parent_map: HashMap<usize, Option<usize>>,
    pub(crate) fringe: HashSet<usize>,
}

impl<'a> CostGuidedTreeSearchResult<'a> {
    pub fn sucess(&self) -> bool {
        self.reached
    }

    pub fn path_to_stop(&self) -> Option<Vec<usize>> {
        self.path_to(self.stop_idx)
    }

    pub fn path_to(&self, goal_idx: usize) -> Option<Vec<usize>> {
        assert!(goal_idx <= self.graph.vertices.len() - 1);
        if goal_idx == self.start_idx {
            return Some(vec![self.start_idx]);
        }
        let mut idx = goal_idx;
        let mut path = vec![idx];
        while let Some(&Some(parent_idx)) = self.parent_map.get(&idx) {
            path.push(parent_idx);
            idx = parent_idx;
        }
        let path: Vec<usize> = path.into_iter().rev().collect();
        match path.len() {
            1 => None,
            _ => Some(path),
        }
    }
}

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

pub trait CostGuidedWaveTreeSearch<Cost: Ord>: Debug + Sized {
    fn as_start(my_vertex_state: Vec3, stop_vertex_state: Vec3) -> Self;

    fn as_adj(
        prev_vertex_state: Vec3,
        my_vertex_state: Vec3,
        stop_vertex_state: Vec3,
        parent: &Self,
    ) -> Self;

    fn cost(&self) -> Cost;

    fn try_on<'a>(
        graph: &'a Graph,
        start_idx: usize,
        stop_idx: usize,
    ) -> CostGuidedTreeSearchResult<'a> {
        Self::try_on_with_alloc(graph, start_idx, stop_idx, 1.0)
    }

    fn try_on_with_alloc<'a>(
        graph: &'a Graph,
        start_idx: usize,
        stop_idx: usize,
        initial_alloc_frac: f32,
    ) -> CostGuidedTreeSearchResult<'a> {
        assert!(start_idx < graph.vertices.len());
        assert!(stop_idx < graph.vertices.len());
        assert!(initial_alloc_frac >= 0.0);
        let start_search_state = Self::as_start(
            graph.vertices[start_idx].state,
            graph.vertices[stop_idx].state,
        );
        let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
        let mut parent_map = HashMap::with_capacity(collec_alloc_size);
        parent_map.insert(start_idx, None);

        let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);

        struct CostOrdAndIndex<Cost: Ord> {
            idx: usize,
            cost: Cost,
        }

        impl<Cost: Ord> PartialEq for CostOrdAndIndex<Cost> {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost
            }
        }

        impl<Cost: Ord> Eq for CostOrdAndIndex<Cost> {}

        impl<Cost: Ord> Ord for CostOrdAndIndex<Cost> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.cost.cmp(&other.cost)
            }
        }

        impl<Cost: Ord> PartialOrd for CostOrdAndIndex<Cost> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        fringe.push(Reverse(CostOrdAndIndex {
            idx: start_idx,
            cost: start_search_state.cost(),
        }));
        let mut tree = HashMap::with_capacity(collec_alloc_size);
        tree.insert(start_idx, start_search_state);
        while let Some(Reverse(CostOrdAndIndex { idx: curr_idx, .. })) = fringe.pop() {
            if curr_idx == stop_idx {
                return CostGuidedTreeSearchResult {
                    graph,
                    start_idx,
                    stop_idx,
                    parent_map,
                    fringe: fringe
                        .into_sorted_vec()
                        .into_iter()
                        .map(|Reverse(CostOrdAndIndex { idx, .. })| idx)
                        .collect(),
                    reached: true,
                };
            }
            for &adj_idx in graph.vertices[curr_idx].adjacencies.iter() {
                if let None = tree.get(&adj_idx) {
                    let adj_search_state = Self::as_adj(
                        graph.vertices[curr_idx].state,
                        graph.vertices[adj_idx].state,
                        graph.vertices[stop_idx].state,
                        &tree[&curr_idx],
                    );
                    parent_map.insert(adj_idx, Some(curr_idx));
                    fringe.push(Reverse(CostOrdAndIndex {
                        idx: adj_idx,
                        cost: adj_search_state.cost(),
                    }));
                    tree.insert(adj_idx, adj_search_state);
                }
            }
        }
        CostGuidedTreeSearchResult {
            graph,
            start_idx,
            stop_idx,
            parent_map,
            fringe: fringe
                .into_sorted_vec()
                .into_iter()
                .map(|Reverse(CostOrdAndIndex { idx, .. })| idx)
                .collect(),
            reached: false,
        }
    }
}

#[derive(Debug)]
pub struct DFS {
    order: isize,
}

impl CostGuidedWaveTreeSearch<isize> for DFS {
    fn as_start(_: Vec3, _: Vec3) -> Self {
        Self { order: -0 }
    }

    fn as_adj(_: Vec3, _: Vec3, _: Vec3, parent: &Self) -> Self {
        Self {
            order: parent.order - 1,
        }
    }

    fn cost(&self) -> isize {
        self.order
    }
}

#[derive(Debug)]
pub struct BFS {
    jumps_from_start: usize,
}

impl CostGuidedWaveTreeSearch<usize> for BFS {
    fn as_start(_: Vec3, _: Vec3) -> Self {
        Self {
            jumps_from_start: 0,
        }
    }

    fn as_adj(_: Vec3, _: Vec3, _: Vec3, parent: &Self) -> Self {
        Self {
            jumps_from_start: parent.jumps_from_start + 1,
        }
    }

    fn cost(&self) -> usize {
        self.jumps_from_start
    }
}

#[derive(Debug)]
pub struct WeightableAStar<const NUM: usize, const DEN: usize> {
    dist_from_start: f32,
    total_cost: f32,
}

impl<const NUM: usize, const DEN: usize> CostGuidedWaveTreeSearch<OrderedFloat<f32>>
    for WeightableAStar<NUM, DEN>
{
    fn as_start(my_vertex_state: Vec3, stop_vertex_state: Vec3) -> Self {
        Self {
            dist_from_start: 0.0,
            total_cost: 0.0 + (my_vertex_state - stop_vertex_state).length(),
        }
    }

    fn as_adj(
        prev_vertex_state: Vec3,
        my_vertex_state: Vec3,
        stop_vertex_state: Vec3,
        parent: &Self,
    ) -> Self {
        let dist_from_start =
            parent.dist_from_start + (prev_vertex_state - my_vertex_state).length();
        Self {
            dist_from_start,
            total_cost: dist_from_start
                + (my_vertex_state - stop_vertex_state).length() * (NUM as f32 / DEN as f32),
        }
    }

    fn cost(&self) -> OrderedFloat<f32> {
        OrderedFloat(self.total_cost)
    }
}

pub type UCS = WeightableAStar<0, 1>;
pub type AStar = WeightableAStar<1, 1>;
pub type AStarWeighted2 = WeightableAStar<2, 1>;

#[derive(Debug)]
pub struct CuboidSpace {
    pub size: Vec3,
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

use bevy::render::mesh::Mesh;

#[derive(Debug)]
pub struct Path {
    pub(crate) vertices: Vec<Vec3>,
}

impl Path {
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl<'a> From<&CostGuidedTreeSearchResult<'a>> for Path {
    fn from(ts: &CostGuidedTreeSearchResult<'a>) -> Path {
        let vertices = match ts.path_to_stop() {
            None => vec![],
            Some(path) => path
                .into_iter()
                .map(|idx| ts.graph.vertices[idx].state)
                .collect(),
        };
        Path { vertices }
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
