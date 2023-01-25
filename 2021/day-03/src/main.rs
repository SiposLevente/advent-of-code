use std::fs;

fn main() {
    let data_array = read_input("puzzle.txt");
    let diagnostics_data = diagnostics(&data_array.0, data_array.1);
    let gamma_rate = diagnostics_data.0;
    let epsilon_rate = diagnostics_data.1;
    let solution = gamma_rate * epsilon_rate;
    println!(
        "Gamma rate: {}\nEpsilon rate: {}\nSolution: {}",
        gamma_rate, epsilon_rate, solution
    );
}

pub fn read_input(puzzle: &str) -> (Vec<usize>, usize) {
    match fs::read_to_string(puzzle) {
        Ok(i) => {
            let len = if let Some(i) = i.lines().next() {
                i.len()
            } else {
                panic!("Not valid input!");
            };

            let mut lines: Vec<usize> = vec![];

            for data in i.lines() {
                let mut real_data = 0;
                for character in 0..len {
                    if data.chars().nth(character).unwrap() == '1' {
                        real_data += 1 << (len - character - 1);
                        println!("{}", real_data);
                    }
                }
                lines.push(real_data);
            }

            (lines, len)
        }
        Err(e) => panic!("{}", e),
    }
}

fn diagnostics(data_array: &Vec<usize>, size: usize) -> (usize, usize) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..size {
        let mask = 1 << i;
        let mut bit_counter = 0;

        for data in data_array {
            if data & mask == mask {
                bit_counter += 1;
            }
        }

        if bit_counter > data_array.len() >> 1 {
            gamma_rate += mask;
        } else {
            epsilon_rate += mask;
        }
    }
    (gamma_rate, epsilon_rate)
}