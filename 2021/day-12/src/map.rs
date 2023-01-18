use std::{collections::HashMap, fmt::Display, fs::read_to_string};

const START_NODE_NAME: &str = "start";
const END_NODE_NAME: &str = "end";

pub struct CaveMap {
    nodes: HashMap<String, Vec<String>>,
    pub routes: Vec<Vec<String>>,
}

impl CaveMap {
    pub fn new(input_text: &str) -> CaveMap {
        CaveMap::generate_from_file(input_text)
    }

    pub fn start_search(&mut self) {
        if self.nodes.contains_key(START_NODE_NAME) {
            CaveMap::search(self, START_NODE_NAME, &mut vec![]);
        } else {
            panic!("Error in routes! No node name start found!");
        }
    }

    pub fn search(map: &mut CaveMap, node_name: &str, visited_nodes: &mut Vec<String>) {
        if node_name == END_NODE_NAME {
            visited_nodes.push(END_NODE_NAME.to_string());
            map.routes.push(visited_nodes.clone());
        } else {
            if map.nodes.contains_key(node_name) {
                if let Some(node_vector) = map.nodes.clone().get(node_name) {
                    visited_nodes.push(node_name.to_string());
                    for node in node_vector {
                        if !visited_nodes.contains(&node.to_string())
                            || CaveMap::is_big_cave(node.to_string())
                        {
                            CaveMap::search(map, &node, &mut visited_nodes.clone());
                        }
                    }
                }
            }
        }
    }

    fn is_big_cave(cave_sign: String) -> bool {
        if let Some(chr) = cave_sign.chars().nth(0) {
            chr.is_uppercase()
        } else {
            panic!("Invalid cave sign!");
        }
    }

    fn generate_from_file(input_text: &str) -> CaveMap {
        let file_content = read_to_string(input_text);
        let mut map = CaveMap {
            routes: vec![],
            nodes: HashMap::new(),
        };

        match file_content {
            Ok(content_string) => {
                for line in content_string.replace('\r', "").split('\n') {
                    let line_content: Vec<&str> = line.split('-').collect();
                    if !map.nodes.contains_key(line_content[0]) {
                        map.nodes.insert(
                            line_content[0].to_string(),
                            vec![line_content[1].to_string()],
                        );
                    } else {
                        if let Some(element) = map.nodes.get_mut(line_content[0]) {
                            element.push(line_content[1].to_string());
                        }
                    }

                    if !map.nodes.contains_key(line_content[1]) {
                        map.nodes.insert(
                            line_content[1].to_string(),
                            vec![line_content[0].to_string()],
                        );
                    } else {
                        if let Some(element) = map.nodes.get_mut(line_content[1]) {
                            if !element.contains(&line_content[0].to_string()) {
                                element.push(line_content[0].to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => panic!("Error reading file: {}", e),
        }

        map
    }
}

impl Display for CaveMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {:?}", self.nodes)
    }
}
