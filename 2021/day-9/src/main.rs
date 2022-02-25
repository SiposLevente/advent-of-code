fn main() {
    let data = vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];

    let is_low_point = |x: usize, y: usize| {
        if x != 0 {
            if data[y][x - 1] < data[y][x] {
                return false;
            }
        }

        if y != 0 {
            if data[y - 1][x] < data[y][x] {
                return false;
            }
        }

        if x != data[0].len() - 1 {
            if data[y][x + 1] < data[y][x] {
                return false;
            }
        }

        if y != data.len() - 1 {
            if data[y + 1][x] < data[y][x] {
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

    let low_points_sum: isize = low_points.iter().map(|x| x + 1).sum();

    println!(
        "Sum of the risk levels of all low points in the heightmap is: {}",
        low_points_sum
    )
}
