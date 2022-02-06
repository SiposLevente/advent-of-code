pub mod line;
pub mod map;
use line::Line;
use map::Map;

fn main() {
    let mut map = Map::new();
    let l1 = Line::new_from_coords(0, 9, 5, 9);
    let l2 = Line::new_from_coords(8, 0, 0, 8);
    let l3 = Line::new_from_coords(9, 4, 3, 4);
    let l4 = Line::new_from_coords(2, 2, 2, 1);
    let l5 = Line::new_from_coords(7, 0, 7, 4);
    let l6 = Line::new_from_coords(6, 4, 2, 0);
    let l7 = Line::new_from_coords(0, 9, 2, 9);
    let l8 = Line::new_from_coords(3, 4, 1, 4);
    let l9 = Line::new_from_coords(0, 0, 8, 8);
    let l10 = Line::new_from_coords(5, 5, 8, 2);
    let lines = vec![l1, l2, l3, l4, l5, l6, l7, l8, l9, l10];

    for mut line in lines {
        for point in line.line_cords() {
            map.mark(point);
        }
    }

    println!("{}",map.to_string());

    println!("Dangerous areas: {}", map.count_dangerous_areas());

}
