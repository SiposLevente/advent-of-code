use std::fs;

fn main() {
    let data_array = get_puzzle("puzzle.txt");

    let mut counter = 0;
    for data in data_array {
        if data.contains('|') {
            counter += data
                .split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
                .count();
        } else {
            panic!("Invalid input!");
        }
    }

    println!("1,4,7,8 only appear {} times!", counter);
}

fn get_puzzle(puzzle: &str) -> Vec<String> {
    match fs::read_to_string(puzzle) {
        Ok(i) => i.lines().map(|x| <&str>::clone(&x).to_string()).collect(),
        Err(e) => panic!("{}", e),
    }
}
