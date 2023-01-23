use std::fs::read_to_string;

use transparent_paper::{FoldInstruction, TransparentPaper};

mod transparent_paper;

fn main() {
    let mut transparent_paper = read_input("input.txt");
    transparent_paper
        .fold_according_instructions(transparent_paper::NumberOfFolds::All);
    println!(
        "{}\nNum of holes: {}",
        transparent_paper,
        transparent_paper.count_dots()
    );
}

fn read_input(input_file: &str) -> TransparentPaper {
    let mut holes: Vec<(usize, usize)> = vec![];
    let mut instructions: Vec<FoldInstruction> = vec![];
    let mut x = 0;
    let mut y = 0;

    for line in read_to_string(input_file).unwrap().lines() {
        if line.contains(',') {
            let coords: Vec<&str> = line.split(',').collect();
            let x_val = coords[0].parse().unwrap();
            let y_val = coords[1].parse().unwrap();
            holes.push((x_val, y_val));
            if x_val > x {
                x = x_val;
            }
            if y_val > y {
                y = y_val;
            }
        } else if line.contains('=') {
            let fold_coords: Vec<&str> = line[11..line.len()].split('=').collect();
            match fold_coords[0] {
                "x" => instructions.push(FoldInstruction::X(fold_coords[1].parse().unwrap())),
                "y" => instructions.push(FoldInstruction::Y(fold_coords[1].parse().unwrap())),
                _ => panic!("Invalid coordinate type!"),
            }
        }
    }
    let mut transparent_paper = TransparentPaper::new(y + 1, x + 1, instructions);
    for hole in holes {
        transparent_paper.add_dot(hole.0, hole.1);
    }

    transparent_paper
}
