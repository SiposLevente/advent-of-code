use point::point::Point;

pub struct Map {
    map: Vec<Vec<isize>>,
}

impl Map {
    pub fn new(size: usize) -> Map {
        Map {
            map: vec![vec![0; size]; size],
        }
    }

    pub fn mark(&mut self, point: Point<isize>) {
        self.map[point.y as usize][point.x as usize] += 1;
    }

    pub fn to_string(&self) -> String {
        let mut map = String::new();

        for y in &self.map {
            for x in y {
                if *x == 0 {
                    map.push_str(".");
                } else {
                    map.push_str(&x.to_string());
                }
            }
            map.push_str("\n");
        }
        map
    }

    pub fn count_dangerous_areas(&self) -> isize {
        let mut counter = 0;

        for y in &self.map {
            for x in y {
                if x >= &2 {
                    counter += 1;
                }
            }
        }

        counter
    }
}
