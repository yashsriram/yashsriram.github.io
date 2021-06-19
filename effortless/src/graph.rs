use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[serde(deny_unknown_fields)]
struct Vertex {
    id: String,
    description: String,
    significance: String,
    proof: String,
}

#[derive(Default)]
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
            adjacency_list.push(Adjacent::default());
        }
        let mut id_to_idx_map = HashMap::new();
        // Validate non-empty, unique ids and topological order
        for (vertex_idx, vertex) in topological_list.iter().enumerate() {
            // Validate non-empty ids until now
            if vertex.id.is_empty() {
                return Err(String::from(format!(
                    "vertex with empty id found at index {}.",
                    vertex_idx
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
            // Validate all parents are already read
            let tokens: Vec<&str> = vertex.description.split('@').collect();
            if tokens.len() % 2 == 0 {
                return Err(String::from(format!(
                    "vertex \"{}\" at index {} has invalid syntax for parent references.",
                    vertex.id, vertex_idx
                )));
            }
            for (token_idx, &token) in tokens.iter().enumerate() {
                if token_idx % 2 == 0 {
                    continue;
                }
                match id_to_idx_map.get(token) {
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
