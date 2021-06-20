use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[serde(deny_unknown_fields)]
struct Vertex {
    id: String,
    description: Vec<String>,
    significance: String,
    proof: String,
}

struct Adjacent {
    parents: Vec<usize>,
    children: Vec<usize>,
}

pub struct DirectedAcyclicGraph {
    topological_list: Vec<Vertex>,
    adjacency_list: Vec<Adjacent>,
    id_to_idx_map: HashMap<String, usize>,
}

impl DirectedAcyclicGraph {
    pub fn new(reader: BufReader<File>) -> Result<DirectedAcyclicGraph, String> {
        // Order of JSON array is preserved during serialization and deserialization
        // Validate proper json format
        let topological_list: Vec<Vertex> = match serde_json::from_reader(reader) {
            Ok(r) => r,
            Err(_) => return Err(String::from("cannot parse vertices from given stream.")),
        };
        let mut adjacency_list: Vec<Adjacent> = Vec::with_capacity(topological_list.len());
        for _ in 0..topological_list.len() {
            adjacency_list.push(Adjacent {
                parents: vec![],
                children: vec![],
            });
        }
        let mut id_to_idx_map = HashMap::new();
        // Validate non-empty, stripped_snake_case, unique ids and topological order
        // Validating above things automatically ensures acyclicity; since every parent of a vertex has to occur before it there are no cycles
        for (vertex_idx, vertex) in topological_list.iter().enumerate() {
            // Validate non-empty ids until now
            if vertex.id.is_empty() {
                return Err(String::from(format!(
                    "vertex with empty id found at index {}.",
                    vertex_idx
                )));
            }
            // Validate stripped_snake_case ids until now
            if let Some(_) = vertex
                .id
                .find(|c: char| c.is_whitespace() || c.is_uppercase())
            {
                return Err(String::from(format!(
                    "vertex with non stripped_snake_case id \"{}\" found at index {}.",
                    vertex.id, vertex_idx
                )));
            }
            // Validate unique ids until now
            if id_to_idx_map.contains_key(&vertex.id) {
                return Err(String::from(format!(
                    "vertex with duplicate id \"{}\" found at index {}.",
                    vertex.id, vertex_idx
                )));
            }
            // Validate topological order until now
            // Validate that all parents of current vertex are already exist
            if vertex.description.len() % 2 == 0 {
                return Err(String::from(format!(
                    "vertex \"{}\" at index {} has invalid syntax for parent references.",
                    vertex.id, vertex_idx
                )));
            }
            for parent in vertex
                .description
                .iter()
                .enumerate()
                .filter(|&(i, _)| i % 2 == 1)
                .map(|(_, odd_token)| odd_token)
            {
                match id_to_idx_map.get(parent) {
                    Some(&parent_idx) => {
                        if adjacency_list[vertex_idx]
                            .parents
                            .iter()
                            .any(|&i| i == parent_idx)
                        {
                            return Err(String::from(format!(
                                "vertex \"{}\" at index {} has reference a parent multiple times.",
                                vertex.id, vertex_idx
                            )));
                        }
                        // Everything okay
                        adjacency_list[vertex_idx].parents.push(parent_idx);
                        adjacency_list[parent_idx].children.push(vertex_idx);
                    }
                    None => {
                        return Err(String::from(format!(
                            "vertex \"{}\" at index {} has reference for an unknown parent.",
                            vertex.id, vertex_idx
                        )));
                    }
                }
            }
            // Everything okay
            id_to_idx_map.insert(vertex.id.clone(), vertex_idx);
        }
        Ok(DirectedAcyclicGraph {
            topological_list: topological_list,
            adjacency_list: adjacency_list,
            id_to_idx_map: id_to_idx_map,
        })
    }
}

pub mod context {
    use super::DirectedAcyclicGraph;
    use rocket::serde::Serialize;

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct StatementContext {
        id: String,
        description: Vec<String>,
        significance: String,
        proof: String,
        parents: Vec<String>,
        children: Vec<String>,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct OpenContext {
        statement: Option<StatementContext>,
        list: Vec<String>,
    }

    impl From<(DirectedAcyclicGraph, &str)> for OpenContext {
        fn from((dag, id): (DirectedAcyclicGraph, &str)) -> Self {
            let statement = match dag.id_to_idx_map.get(id) {
                Some(&idx) => {
                    let (v, a) = (&dag.topological_list[idx], &dag.adjacency_list[idx]);
                    Some(StatementContext {
                        id: v.id.clone(),
                        description: v.description.clone(),
                        significance: v.significance.clone(),
                        proof: v.proof.clone(),
                        parents: a
                            .parents
                            .iter()
                            .map(|&idx| dag.topological_list[idx].id.clone())
                            .collect(),
                        children: a
                            .children
                            .iter()
                            .map(|&idx| dag.topological_list[idx].id.clone())
                            .collect(),
                    })
                }
                None => None,
            };
            let list = dag.topological_list.iter().map(|v| v.id.clone()).collect();
            OpenContext {
                statement: statement,
                list: list,
            }
        }
    }
}
