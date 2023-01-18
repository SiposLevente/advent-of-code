use std::{collections::HashMap, fmt::Display, fs::read_to_string};

const START_NODE_NAME: &str = "start";
const END_NODE_NAME: &str = "end";

pub struct CaveMap {
    nodes: HashMap<String, Vec<String>>,
    routes: Vec<String>,
}

impl CaveMap {
    pub fn new(input_text: &str) -> CaveMap {
        CaveMap::generate_from_file(input_text)
    }

    pub fn start_search(self) {
        if self.nodes.contains_key(START_NODE_NAME) {
            let mut route_string = String::from(START_NODE_NAME);
            self.search(route_string, START_NODE_NAME);
        } else {
            panic!("Error in routes! No node name start found!");
        }
    }

    pub fn search(mut self, route_string: String, node_name: &str) {
        if node_name == END_NODE_NAME {
            self.routes.push(route_string);
        } else {

            if self.nodes.contains_key(node_name){

            } else {
                
            }


            if let Some(start_node_vector) = self.nodes.get(node_name) {
                
            } else {
                panic!("Error in routes! No node name start found!");
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
