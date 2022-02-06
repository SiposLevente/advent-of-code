use std::fs;

fn main() {
    let data_array: Vec<i32> = read_input("puzzle.txt");

    println!(
        "There are {} measurements that are larger than the previous.",
        sonar_sweep(&data_array)
    );
}

fn read_input(puzzle: &str) -> Vec<i32> {
    match fs::read_to_string(puzzle) {
        Ok(i) => {
            let lines: Vec<i32> = i.lines().map(|x| x.parse().unwrap()).collect();
            lines
        }
        Err(e) => panic!("{}", e),
    }
}

fn sonar_sweep(data_array: &Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut prev_data = data_array[0];

    for num in &data_array[1..] {
        if prev_data < *num {
            counter += 1;
        }
        prev_data = *num;
    }

    counter
}
