use super::graph::*;
use bevy::prelude::*;
use ordered_float::OrderedFloat;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Resource, Default)]
pub struct CostGuidedTreeSearchResult {
    pub start_idx: usize,
    pub stop_idx: usize,
    pub reached: bool,
    pub parent_map: HashMap<usize, Option<usize>>,
    pub fringe: HashSet<usize>,
}

impl<'a> CostGuidedTreeSearchResult {
    pub fn path_to_stop(&self, graph: &'a Graph) -> Option<Vec<usize>> {
        assert!(self.stop_idx <= graph.vertices.len() - 1);
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

pub trait CostGuidedWaveTreeSearch<Cost: Ord>: Sized {
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
    ) -> CostGuidedTreeSearchResult {
        assert!(start_idx < graph.vertices.len());
        assert!(stop_idx < graph.vertices.len());
        let start_search_state =
            Self::as_start(graph.vertices[start_idx].pos, graph.vertices[stop_idx].pos);
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
                        graph.vertices[curr_idx].pos,
                        graph.vertices[adj_idx].pos,
                        graph.vertices[stop_idx].pos,
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

#[derive(Resource, Default)]
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

#[derive(Resource, Default)]
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

#[derive(Resource, Default)]
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
