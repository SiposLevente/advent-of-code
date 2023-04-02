use std::fmt::Display;

#[derive(Debug)]
pub struct MapElement {
    pub value: usize,
    pub route_cost: usize,
}

impl MapElement {
    pub fn new(value: usize) -> MapElement {
        MapElement {
            value,
            route_cost: 0,
        }
    }
}

impl Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Value: {}; Cost: {} }}", self.value, self.route_cost)
    }
}
