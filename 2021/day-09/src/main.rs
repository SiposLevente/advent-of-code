use core::panic;

fn get_data(file: &str) -> Vec<Vec<isize>> {
    let data: String = if let Ok(i) = std::fs::read_to_string(file) {
        i
    } else {
        panic!("Error reading file!");
    };

    let mut return_vector: Vec<Vec<isize>> = vec![];

    for line in data.lines() {
        let mut tmp_vector: Vec<isize> = vec![];
        for chars in line.chars() {
            tmp_vector.push(chars.to_string().parse::<isize>().unwrap())
        }
        return_vector.push(tmp_vector)
    }

    return_vector
}

fn main() {
    // let data = vec![
    //     vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
    //     vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
    //     vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
    //     vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
    //     vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    // ];

    let data = get_data("puzzle.txt");

    let is_low_point = |x: usize, y: usize| {
        let curr_data = data[y][x];
        
        if x != 0 {
            let adj_data = data[y][x - 1];
            if adj_data == curr_data || adj_data < curr_data {
                return false;
            }
        }
        if y != 0 {
            let adj_data = data[y - 1][x];
            if adj_data == curr_data || adj_data < curr_data {
                return false;
            }
        }
        if x != data[0].len() - 1 {
            let adj_data = data[y][x + 1];
            if adj_data == curr_data || adj_data < curr_data {
                return false;
            }
        }
        if y != data.len() - 1 {
            let adj_data = data[y + 1][x];
            if adj_data == curr_data || adj_data < curr_data {
                return false;
            }
        }
        true
    };

    let mut low_points = vec![];

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if is_low_point(x, y) {
                low_points.push(data[y][x]);
            }
        }
    }

    print!("{:?}\n", low_points);
    let low_points_sum: isize = low_points.iter().map(|x| x + 1).sum();

    println!(
        "Sum of the risk levels of all low points in the heightmap is: {}",
        low_points_sum
    )
}
