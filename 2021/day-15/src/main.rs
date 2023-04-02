use std::fs::read_to_string;

use map_element::MapElement;

mod map_element;

fn main() {
    let mut map = read_input("puzzle.txt");
    println!("{}", calculate_shortest_path_value(&mut map));
}

fn calculate_shortest_path_value(map: &mut Vec<Vec<MapElement>>) -> usize {
    let mut element_counter: isize = 2;

    for row in 0..map.len() as isize - 1 {
        let mut x: isize = row + 1;
        let mut y = 0;

        for _ in 0..element_counter {
            if x - 1 < 0 {
                map[x as usize][y as usize].route_cost =
                    map[x as usize][y as usize].value + map[x as usize][y as usize - 1].route_cost;
            } else if y - 1 < 0 {
                map[x as usize][y as usize].route_cost =
                    map[x as usize][y as usize].value + map[x as usize - 1][y as usize].route_cost;
            } else {
                let left_route_cost = map[x as usize - 1][y as usize].route_cost;
                let top_route_cost = map[x as usize][y as usize - 1].route_cost;

                map[x as usize][y as usize].route_cost = if left_route_cost < top_route_cost {
                    map[x as usize][y as usize].value + left_route_cost
                } else {
                    map[x as usize][y as usize].value + top_route_cost
                };
            }

            x -= 1;
            y += 1;
        }

        element_counter += 1;
    }
    element_counter = map.len() as isize - 1;

    for row in 0..map.len() as isize {
        let mut x: isize = map.len() as isize - 1;
        let mut y: isize = row + 1;

        for _ in 0..element_counter {
            if x - 1 < 0 {
                map[x as usize][y as usize].route_cost =
                    map[x as usize][y as usize].value + map[x as usize][y as usize - 1].route_cost;
            } else if y - 1 < 0 {
                map[x as usize][y as usize].route_cost =
                    map[x as usize][y as usize].value + map[x as usize - 1][y as usize].route_cost;
            } else {
                let left_route_cost = map[x as usize - 1][y as usize].route_cost;
                let top_route_cost = map[x as usize][y as usize - 1].route_cost;

                map[x as usize][y as usize].route_cost = if left_route_cost < top_route_cost {
                    map[x as usize][y as usize].value + left_route_cost
                } else {
                    map[x as usize][y as usize].value + top_route_cost
                };
            }

            x -= 1;
            y += 1;
        }
        element_counter -= 1;
    }

    map[map.len() - 1][map[0].len() - 1].route_cost
}

fn read_input(puzzle_file: &str) -> Vec<Vec<MapElement>> {
    let mut ret_vector = vec![];
    if let Ok(file) = read_to_string(puzzle_file) {
        for (index, lines) in file.replace('\r', "").split('\n').enumerate() {
            ret_vector.push(vec![]);
            for character in lines.chars() {
                ret_vector[index].push(MapElement::new(
                    character.to_digit(10).expect("Incorrect puzzle input!") as usize,
                ));
            }
        }
    } else {
        panic!("Error while reading the file!");
    }
    ret_vector[0][0].route_cost = 0;

    ret_vector
}
