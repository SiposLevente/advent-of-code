fn main() {
    let days_to_observe = 80;
    let mut school = vec![3, 4, 3, 1, 2];
    println!("Initial day:\t{:?}", school);

    for i in 1..=days_to_observe {
        for fish in 0..school.len() {
            school[fish] -= 1;
            if school[fish] < 0 {
                school[fish] = 6;
                school.push(8);
            }
        }

        if i == 1 {
            println!("After {} day:\t{:?}", i, school);
        } else {
            println!("After {} days:\t{:?}", i, school);
        }
    }

    println!(
        "After {} days, there are {} lanternfish!",
        days_to_observe,
        school.len()
    );
}
