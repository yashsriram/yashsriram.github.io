use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
    const PARENT_DELIMITER: char = '@';

    pub fn from_file(path: PathBuf) -> Result<DirectedAcyclicGraph, String> {
        let content = std::fs::read_to_string(path).or_else(|e| Err(e.to_string()))?;
        // Order of JSON array is preserved during serialization and deserialization
        // Validate proper json format
        let topological_list: Vec<Vertex> = match serde_json::from_str(&content) {
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
                    "vertex \"{}\" at index {} has invalid syntax for parent references. has even length description array.",
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

    pub fn add_vertex(
        &mut self,
        id: &str,
        description: &str,
        significance: &str,
        proof: &str,
    ) -> Result<(), String> {
        // Validate non-empty id
        if id.is_empty() {
            return Err(String::from("empty id."));
        }
        // Validate stripped_snake_case id
        if let Some(_) = id.find(|c: char| c.is_whitespace() || c.is_uppercase()) {
            return Err(String::from("non stripped_snake_case id."));
        }
        // Validate unique id
        if self.id_to_idx_map.contains_key(id) {
            return Err(String::from("duplicate id."));
        }
        // Validate topological order
        // Validate that all parents of current vertex are already exist
        let tokens: Vec<&str> = description
            .split(DirectedAcyclicGraph::PARENT_DELIMITER)
            .collect();
        if tokens.len() % 2 == 0 {
            return Err(String::from(
                "has invalid syntax for parent references. too many or too less splitting tokens.",
            ));
        }
        let mut adjacency = Adjacent {
            parents: vec![],
            children: vec![],
        };
        for parent in tokens
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, &odd_token)| odd_token)
        {
            match self.id_to_idx_map.get(parent) {
                Some(&parent_idx) => {
                    if adjacency.parents.iter().any(|&i| i == parent_idx) {
                        return Err(String::from("has reference a parent multiple times."));
                    }
                    adjacency.parents.push(parent_idx);
                }
                None => {
                    return Err(String::from("has reference for an unknown parent."));
                }
            }
        }
        // Everything okay
        let vertex = Vertex {
            id: String::from(id),
            description: tokens.iter().map(|&t| String::from(t)).collect(),
            significance: String::from(significance),
            proof: String::from(proof),
        };
        let vertex_idx = self.topological_list.len();
        // Add to id_to_idx_map
        self.id_to_idx_map.insert(vertex.id.clone(), vertex_idx);
        // Add to topological_list
        self.topological_list.push(vertex);
        // Add to adjacency_list
        for &parent_idx in adjacency.parents.iter() {
            self.adjacency_list[parent_idx].children.push(vertex_idx);
        }
        self.adjacency_list.push(adjacency);
        Ok(())
    }

    pub fn save_to_file(&self, path: PathBuf) -> Result<(), String> {
        let content_string =
            serde_json::to_string_pretty(&self.topological_list).or_else(|e| Err(e.to_string()))?;
        std::fs::write(path, content_string).or_else(|e| Err(e.to_string()))
    }

    pub fn list_ids(&self) -> Vec<String> {
        self.topological_list.iter().map(|v| v.id.clone()).collect()
    }
}

pub mod context {
    use super::DirectedAcyclicGraph;
    use crate::CreateForm;
    use rocket::form::{Form, Strict};
    use rocket::serde::Serialize;
    use std::convert::TryFrom;

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct StatementContext {
        id: String,
        description: Vec<String>,
        significance: String,
        proof: String,
        parents: Vec<String>,
        children: Vec<String>,
        next_id: String,
        prev_id: String,
    }

    impl TryFrom<(&DirectedAcyclicGraph, &str)> for StatementContext {
        type Error = &'static str;

        fn try_from((dag, id): (&DirectedAcyclicGraph, &str)) -> Result<Self, Self::Error> {
            match dag.id_to_idx_map.get(id) {
                Some(&idx) => {
                    let (v, a) = (&dag.topological_list[idx], &dag.adjacency_list[idx]);
                    let next_idx = if idx + 1 == dag.topological_list.len() {
                        idx
                    } else {
                        idx + 1
                    };
                    let prev_idx = if idx == 0 { 0 } else { idx - 1 };
                    Ok(StatementContext {
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
                        next_id: dag.topological_list[next_idx].id.clone(),
                        prev_id: dag.topological_list[prev_idx].id.clone(),
                    })
                }
                None => Err("invalid id."),
            }
        }
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct OpenContext {
        statement: Option<StatementContext>,
        list: Vec<String>,
    }

    impl From<(DirectedAcyclicGraph, &str)> for OpenContext {
        fn from((dag, id): (DirectedAcyclicGraph, &str)) -> Self {
            let statement = match StatementContext::try_from((&dag, id)) {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            let list = dag.list_ids();
            OpenContext {
                statement: statement,
                list: list,
            }
        }
    }

    #[derive(Serialize, Default)]
    #[serde(crate = "rocket::serde")]
    pub struct CreateContext {
        id: String,
        description: String,
        significance: String,
        proof: String,
        msg: String,
        list: Vec<String>,
    }

    impl From<DirectedAcyclicGraph> for CreateContext {
        fn from(dag: DirectedAcyclicGraph) -> Self {
            CreateContext {
                list: dag.list_ids(),
                ..Default::default()
            }
        }
    }

    impl From<(Form<Strict<CreateForm<'_>>>, String, DirectedAcyclicGraph)> for CreateContext {
        fn from(
            (nv, msg, dag): (Form<Strict<CreateForm<'_>>>, String, DirectedAcyclicGraph),
        ) -> Self {
            CreateContext {
                id: String::from(nv.id),
                description: String::from(nv.description),
                significance: String::from(nv.significance),
                proof: String::from(nv.proof),
                msg: msg,
                list: dag.list_ids(),
            }
        }
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct VertexContext {
        id: String,
        color: String,
        opaqueness: String,
        description: Vec<String>,
        parents: Vec<String>,
        children: Vec<String>,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct GraphContext {
        statements: Vec<VertexContext>,
    }

    impl From<DirectedAcyclicGraph> for GraphContext {
        fn from(dag: DirectedAcyclicGraph) -> Self {
            fn heat_map(min: f32, val: f32, max: f32) -> String {
                assert!(min < max);
                let gray = ((val - min) / (max - min)) * 255.0;
                let gray = if gray < 0.0 {
                    0.0
                } else if gray > 255.0 {
                    255.0
                } else {
                    gray
                };
                let gray = gray as u8;
                format!("{:02X}", gray)
            }
            GraphContext {
                statements: dag
                    .topological_list
                    .iter()
                    .zip(dag.adjacency_list.iter())
                    .map(|(v, a)| VertexContext {
                        id: v.id.clone(),
                        color: String::from(format!(
                            "#{:02X}{:02X}{:02X}",
                            fastrand::u8(..),
                            fastrand::u8(..),
                            fastrand::u8(..)
                        )),
                        opaqueness: heat_map(0.0, a.parents.len() as f32, 5.0),
                        description: v.description.clone(),
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
                    .collect::<Vec<VertexContext>>(),
            }
        }
    }
}
