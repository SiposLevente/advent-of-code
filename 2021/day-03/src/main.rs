fn main() {
    let data_array: Vec<i32> = vec![
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let diagnostics_data = diagnostics(&data_array);
    let gamma_rate = diagnostics_data.0;
    let epsilon_rate = diagnostics_data.1;
    let solution = gamma_rate * epsilon_rate;
    println!(
        "Gamma rate: {}\nEpsilon rate: {}\nSolution: {}",
        gamma_rate, epsilon_rate, solution
    );
}

fn diagnostics(data_array: &Vec<i32>) -> (i32, i32) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..5 {
        let mask = 1 << i;
        let mut gamma_rate_counter = 0;
        for data in data_array {
            if data & mask == mask {
                gamma_rate_counter += 1;
            }
        }
        if gamma_rate_counter >= data_array.len() / 2 {
            gamma_rate += mask;
        } else {
            epsilon_rate += mask;
        }
    }
    (gamma_rate, epsilon_rate)
}
