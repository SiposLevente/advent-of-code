use crate::bingo_board::Board;
use std::fs;

mod bingo_board;

fn main() {
    let data_array = read_bingo_number("bingo_numbers.txt");

    let bingo_array: Vec<Vec<usize>> = read_bingo_boards("bingo_boards.txt", 5);
    let mut bingo_cards: Vec<Board> = vec![];

    for i in bingo_array {
        bingo_cards.push(Board::new(Board::convert_to_bingo_numbers(&i), 5));
    }

    let mut game_won = false;
    let mut num_counter = 0;

    while !game_won && num_counter < data_array.len() {
        let mut card_counter = 0;
        while !game_won && card_counter < bingo_cards.len() {
            let coord = bingo_cards[card_counter].mark_num(data_array[num_counter]);
            if let Some(xy) = coord {
                if bingo_cards[card_counter].check_bingo(xy.0, xy.1) {
                    game_won = !game_won;

                    let sum = bingo_cards[card_counter].sum_non_marked_numbers();
                    println!("Sum of non marked numbers: {}", sum);
                    println!(
                        "Number that has been just called: {}",
                        data_array[num_counter]
                    );
                    println!("Solution: {}", sum * data_array[num_counter]);
                }
            }
            card_counter += 1;
        }

        num_counter += 1;
    }
}

fn read_bingo_number(bingo_number_file: &str) -> Vec<usize> {
    let numbers = if let Ok(i) = fs::read_to_string(bingo_number_file) {
        i
    } else {
        panic!("Failed to read file!");
    };

    numbers.split(',').map(|x| x.parse().unwrap()).collect()
}

fn read_bingo_boards(bingo_board_file: &str, bingo_size: usize) -> Vec<Vec<usize>> {
    let boards = if let Ok(i) = fs::read_to_string(bingo_board_file) {
        i
    } else {
        panic!("Failed to read file!");
    };

    let mut num_collection: Vec<usize> = vec![];
    for (counter,i) in boards.lines().enumerate() {
        match counter % (bingo_size + 1) {
            0 | 1 | 2 | 3 | 4 => {
                let nums: Vec<usize> = i
                    .replace("  ", " ")
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect();
                for num in nums {
                    num_collection.push(num);
                }
            }

            _ => (),
        }
    }

    let mut bingo_boards: Vec<Vec<usize>> = vec![vec![]];
    let mut vector_counter = 0;
    let mut counter = 0;

    for i in num_collection {
        if counter < bingo_size * bingo_size {
            bingo_boards[vector_counter].push(i);
        } else {
            bingo_boards.push(vec![]);
            vector_counter += 1;
            bingo_boards[vector_counter].push(i);
            counter = 0;
        }
        counter += 1;
    }

    bingo_boards
}
