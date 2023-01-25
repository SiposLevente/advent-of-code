use std::fs;

fn main() {
    let days_to_observe = 80;
    //let days_to_observe = 256;
    let mut school = read_puzzle("puzzle.txt");
    //println!("Initial day:\t{:?}", school);

    for _i in 1..=days_to_observe {
        for fish in 0..school.len() {
            school[fish] -= 1;
            if school[fish] < 0 {
                school[fish] = 6;
                school.push(8);
            }
        }

        // if i == 1 {
        //     println!("After {} day:\t{:?}", i, school);
        // } else {
        //     println!("After {} days:\t{:?}", i, school);
        // }
    }

    println!(
        "After {} days, there are {} lanternfish!",
        days_to_observe,
        school.len()
    );
}

fn read_puzzle(file_path: &str) -> Vec<isize> {
    let data = if let Ok(i) = fs::read_to_string(file_path) {
        i
    } else {
        panic!("Error reading file!");
    };

    if let Some(i) = data.lines().next() {
        i.split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim().parse().unwrap())
            .collect()
    } else {
        panic!("Invalid input!");
    }
}
