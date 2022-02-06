fn main() {
    let data_array: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    println!("There are {} measurements that are larger than the previous.", sonar_sweep(&data_array));
    for data in data_array {
        println!("{}",data);
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
