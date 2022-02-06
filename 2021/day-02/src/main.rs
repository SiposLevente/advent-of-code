// forward X increases the horizontal position by X units.
// down X increases the depth by X units.
// up X decreases the depth by X units.

fn main() {
    let data_array = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let position = submarine_commands(&data_array).0;

    let depth = submarine_commands(&data_array).1;

    let val = position * depth;

    println!("position: {}\ndepth: {}\nSolution value: {}", position, depth, val);
}

fn submarine_commands(data_array: &Vec<&str>) -> (i32, i32) {
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
            "forward" => {position += unit;}
            "up" => {depth -= unit;}
            "down" => {depth += unit;}
            _ => panic!("Error in direction"),
        };
    }
    (position, depth)
}
