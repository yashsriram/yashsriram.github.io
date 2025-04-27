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
use bricks::*;

mod graph;
use graph::*;

mod spaces;
use spaces::*;

mod search;
use search::*;

mod path;
use path::*;

#[derive(Resource, Default)]
struct Searches {
    searches: Vec<CostGuidedTreeSearchResult>,
}

#[derive(Resource, Default)]
struct Paths {
    paths: Vec<Path>,
}

bricks::simple_vis!(
    "prm search",
    {
        CuboidWithHoldSpace -> draw_space,
        Graph -> draw_graph,
        Searches -> draw_searches,
        Paths -> draw_paths,
    }
);

fn init(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 15.).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));
}

fn on_mouse_click(In(point): In<Result<Vec2, ()>>) {}

fn on_spacebar_press(
    mut space: ResMut<CuboidWithHoldSpace>,
    mut graph: ResMut<Graph>,
    mut searches: ResMut<Searches>,
    mut paths: ResMut<Paths>,
) {
    space.size = Vec3::new(2.1, 2.1, 2.);
    space.hole_radius = 1.;
    graph.sample(&space, 5000, 0.22);
    let [a, b] = graph.add([Vec3::new(0.3, 0.7, 0.5), Vec3::new(-0.5, -1.0, -0.4)], 0.2);
    searches.searches = vec![
        DFS::try_on(&graph, a, b),
        BFS::try_on(&graph, a, b),
        UCS::try_on(&graph, a, b),
        AStar::try_on(&graph, a, b),
        WeightableAStar::<11, 10>::try_on(&graph, a, b),
        AStarWeighted2::try_on(&graph, a, b),
    ];
    paths.paths.clear();
    for search in searches.searches.iter() {
        let mut path = Path::default();
        path.generate_for(&graph, &search);
        paths.paths.push(path);
    }
}

fn draw_space(mut gizmos: Gizmos, space: Res<CuboidWithHoldSpace>) {
    gizmos.cuboid(Transform::from_scale(space.size), Color::WHITE);
}

fn draw_graph(mut gizmos: Gizmos, space: Res<CuboidWithHoldSpace>, graph: Res<Graph>) {
    for vertex in &graph.vertices {
        for adj in &vertex.adjacencies {
            let self_position = vertex.state;
            let adj_position = graph.vertices[*adj].state;
            gizmos.line(
                // Draw graph above space
                self_position,
                adj_position,
                Color::srgba(0.035, 0.961, 0.361, 0.1),
            );
        }
    }
}

fn draw_searches(
    mut gizmos: Gizmos,
    space: Res<CuboidWithHoldSpace>,
    graph: Res<Graph>,
    searches: Res<Searches>,
) {
    for (idx, search) in searches.searches.iter().enumerate() {
        for (&child_idx, parent_idx) in search.parent_map.iter() {
            let parent_idx = parent_idx.unwrap_or(child_idx);
            let color = if search.start_idx == parent_idx {
                Color::srgb(1., 1., 0.)
            } else if search.stop_idx == child_idx {
                Color::srgb(1., 0., 1.)
            } else {
                Color::srgba(0.941, 0.051, 0.922, 0.5)
            };
            gizmos.line(
                // Draw search above space and graph
                graph.vertices[child_idx].state + (idx as f32 + 1.) * Vec3::X * (space.size.x + 1.),
                graph.vertices[parent_idx].state
                    + (idx as f32 + 1.) * Vec3::X * (space.size.x + 1.),
                color,
            );
        }
        gizmos.cuboid(
            Transform::from_translation(
                graph.vertices[search.start_idx].state
                    + (idx as f32 + 1.) * Vec3::X * (space.size.x + 1.),
            )
            .with_scale(Vec3::ONE * 0.05),
            Color::srgb(1., 0., 0.),
        );
        gizmos.cuboid(
            Transform::from_translation(
                graph.vertices[search.stop_idx].state
                    + (idx as f32 + 1.) * Vec3::X * (space.size.x + 1.),
            )
            .with_scale(Vec3::ONE * 0.05),
            Color::srgb(0., 1., 0.),
        );
    }
}

fn draw_paths(mut gizmos: Gizmos, space: Res<CuboidWithHoldSpace>, paths: Res<Paths>) {
    for (idx, path) in paths.paths.iter().enumerate() {
        gizmos.linestrip(
            path.vertices.iter().map(|pt| {
                pt + Vec3::Y * space.size.y + Vec3::X * (idx as f32 + 1.) * (space.size.x + 1.)
            }),
            Color::srgb(0.902, 0.843, 0.114),
        );
    }
}
