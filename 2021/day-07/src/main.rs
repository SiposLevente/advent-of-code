#[derive(Debug)]
struct Position {
    position: isize,
    fuel_cost: isize,
}

impl Position {
    fn new(position: isize, fuel_cost: isize) -> Position {
        Position {
            position,
            fuel_cost,
        }
    }
}

fn main() {
    let crab_positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let mut starter_fuel_cost = 0;
    if let Some(i) = calculate_fuel_cost(&crab_positions, 0, -1) {
        starter_fuel_cost = i;
    }

    let min_val = *crab_positions.iter().min().unwrap();
    let max_val = *crab_positions.iter().max().unwrap();

    let mut optimal_pos = Position::new(0, starter_fuel_cost);

    for i in min_val..=max_val {
        if let Some(fuel_cost) = calculate_fuel_cost(&crab_positions, i, optimal_pos.fuel_cost) {
            optimal_pos.fuel_cost = fuel_cost;
            optimal_pos.position = i;
        } else {
            println!("{} position is not optimal!", i);
        }
    }

    println!(
        "Optimal position is {}. Fuel cost for crabs is {}.",
        optimal_pos.position, optimal_pos.fuel_cost
    );
}

fn calculate_fuel_cost(array: &Vec<isize>, position: isize, compare_val: isize) -> Option<isize> {
    let mut cost = 0;
    let mut counter = 0;
    let mut stop = false;

    while counter < array.len() && !stop {
        cost += isize::abs(position - array[counter]);
        if cost > compare_val && compare_val != -1 {
            stop = true;
            return None;
        }
        counter += 1;
    }
    Some(cost)
}
