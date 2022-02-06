use std::fs;

fn main() {
    let data_array = read_input("puzzle.txt");

    let position = submarine_commands(&data_array).0;

    let depth = submarine_commands(&data_array).1;

    let val = position * depth;

    println!(
        "position: {}\ndepth: {}\nSolution value: {}",
        position, depth, val
    );
}

fn read_input(puzzle: &str) -> Vec<String> {
    match fs::read_to_string(puzzle) {
        Ok(i) => i.lines().map(|x| x.clone().to_string()).collect(),
        Err(e) => panic!("{}", e),
    }
}

fn submarine_commands(data_array: &Vec<String>) -> (i32, i32) {
    let mut position = 0;
    let mut depth = 0;

    for data in data_array {
        let mut iter = data.split_whitespace();
        let direction = match iter.next() {
            Some(i) => i,
            None => panic!("Error in the direction"),
        };

        let unit: i32 = iter
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Error in parsing");

        match direction {
            "forward" => {
                position += unit;
            }
            "up" => {
                depth -= unit;
            }
            "down" => {
                depth += unit;
            }
            _ => panic!("Error in direction"),
        };
    }
    (position, depth)
}
