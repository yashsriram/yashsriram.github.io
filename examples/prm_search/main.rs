/// - `Vis`
/// - `State`, `StateSpace`
/// - `Graph` on a `StateSpace`; Vertex = (`StateSpace::State`,  `Set<VertexIdx>`)
/// - `TreeSearch` on `Graph`
///     - [x] start, stop, goal, max idxs
///     - [x] handle stop not reached -> reachable subgraph explored
///     - [x] Tree Search = Open least cost on fringe + Propagate to unexplored adjacencies and add them to fringe
///     - [x] Propagate trait = search state + cost priority + common search fn
///     - [x] CostPriorityWithIndex = Ord on cost + open min cost first
///         - Default impl use : NaN cost is INF + NAN cost = NAN cost
///     - [x] Searching for a stop = may finds path to stop + may some other vertices => so same search can be used to find paths to multiple vertices
///     - [x] Multiple searches on a graph = State per search = No resetting of state
///     - [x] Parallelizable searches
///     - [x] Large graph - small area search is inexpensive - uses sparse seach state using hashmaps (Control initial alloc size of tree and fringe)
///     - [x] Get path to a goal, get path to stop, store start, stop, max idxs
///     - [x] Remove Clone trait bound on vertex search state by merging Propagate and CostPriority => Get reference of underlying graph
/// - `PRM` on `StateSpace`
///     - [x] Create a `Graph<StateSpace>`
///     - [x] Sampling from `StateSpace`
///     - [x] Connecting `Vertices<State>` using dist() trait fn and edge len
///     - [x] Multi (agent) searchable from `Graph`
use bevy::prelude::*;
// use bevy_flycam::PlayerPlugin;
use ordered_float::OrderedFloat;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

mod vis;
use vis::*;

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
        Graph {
            vertices: state_samples
                .into_iter()
                .zip(adjacencies.into_iter())
                .map(|(state, adjacencies)| Vertex { state, adjacencies })
                .collect(),
        }
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
    pub fn path_to_stop(&self) -> Option<Vec<usize>> {
        assert!(self.stop_idx <= self.graph.vertices.len() - 1);
        if self.stop_idx == self.start_idx {
            return Some(vec![self.start_idx]);
        }
        let mut idx = self.stop_idx;
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
        assert!(start_idx < graph.vertices.len());
        assert!(stop_idx < graph.vertices.len());
        let start_search_state = Self::as_start(
            graph.vertices[start_idx].state,
            graph.vertices[stop_idx].state,
        );
        let collec_alloc_size = graph.vertices.len() as usize;
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (600., 300.).into(),
                canvas: Some("#interactive".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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
                let mut graph = Graph::with_num_samples(&space, 2000, 1.0);
                let [a, b] = graph.add([Vec3::new(0.3, 0.7, 0.5), Vec3::new(9.5, 7.3, 4.0)], 1.0);
                let searches = [
                    DFS::try_on(&graph, a, b),
                    BFS::try_on(&graph, a, b),
                    UCS::try_on(&graph, a, b),
                    AStar::try_on(&graph, a, b),
                    WeightableAStar::<11, 10>::try_on(&graph, a, b),
                    AStarWeighted2::try_on(&graph, a, b),
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
                    mesh: meshes.add(Mesh::from(&graph.graph)).into(),
                    material: materials.add(SimpleMaterial {}),
                    ..Default::default()
                });
            },
        )
        .run();
}
