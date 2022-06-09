mod lanternfish;
use lanternfish::Lanternfish;

fn main() {
    let initial_values = vec![3, 4, 3, 1, 2];
    let mut school = Lanternfish::create_school_of_lanternfish(initial_values);
    print!("Initial state:\t\t");
    for fish in 0..school.len() - 1 {
        print!("{},", school[fish]);
    }
    print!("{}\n", school[school.len() - 1]);

    let days_to_run = 80;
    for i in 1..=days_to_run {
        print!("After\t{}\tdays:\t", i);
        for fish in 0..school.len() {
            if school[fish].decrease_timer() {
                let new_fish = Lanternfish::new(8);
                school.push(new_fish);
            }
        }
        for fish in 0..school.len() - 1 {
            print!("{},", school[fish]);
        }
        print!("{}\n", school[school.len() - 1]);
    }

    println!("After {} days there are: {} fish",days_to_run, school.len());
}
