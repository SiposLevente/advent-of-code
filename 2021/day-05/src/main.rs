pub mod line;
pub mod map;

use std::error::Error;
use line::Line;
use map::Map;

fn main() {
    let mut map = Map::new(1000);
    let lines = if let Ok(i) = read_data("puzzle.txt") {
        i
    } else {
        panic!("Invalid file!");
    };

    for mut line in lines {
        for point in line.line_cords() {
            map.mark(point);
        }
    }

    //println!("{}", map.to_string());

    println!("Dangerous areas: {}", map.count_dangerous_areas());
}

fn read_data(file: & str) -> Result<Vec<Line>, Box<dyn Error>> {
    let data_string = std::fs::read_to_string(file)?;
    let mut lines: Vec<Line> = vec![];

    let touple_vector: Vec<(&str, &str)> = data_string
        .lines()
        .map(|x| x.split_once(" -> ").unwrap())
        .collect();

    for i in touple_vector {
        let new_p1_str = i.0.split_once(',').expect("Invalid data!");
        let new_p2_str = i.1.split_once(',').expect("Invalid data!");

        let p1x = new_p1_str.0.parse()?;
        let p1y = new_p1_str.1.parse()?;
        let p2x = new_p2_str.0.parse()?;
        let p2y = new_p2_str.1.parse()?;

        if p1x == p2x || p1y == p2y {
            lines.push(Line::new_from_coords(p1x, p1y, p2x, p2y));
        }
    }

    Ok(lines)
}
