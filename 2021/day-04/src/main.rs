use crate::bingo_board::Board;

mod bingo_board;

fn main() {
    println!("Hello, world!");
    let data_array = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    let card1_nums = [
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ];
    let card1 = Board::new(Board::convert_to_bingo_numbers(&card1_nums), 5);

    let card2_nums = [
        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6,
    ];
    let card2 = Board::new(Board::convert_to_bingo_numbers(&card2_nums), 5);

    let card3_nums = [
        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
    ];
    let card3 = Board::new(Board::convert_to_bingo_numbers(&card3_nums), 5);

    let mut bingo_cards = [card1, card2, card3];

    // for num in data_array {
    //     for card in &mut bingo_cards {
    //         let coord = card.mark_num(num);
    //         if let Some(xy) = coord {
    //             if card.check_bingo(xy.0, xy.1) {
    //                 let sum = card.sum_non_marked_numbers();
    //                 println!("Sum of non marked numbers: {}", sum);
    //                 println!("Number that has been just called: {}", &num);
    //                 println!("Solution: {}", sum * &num);
    //             }
    //         }
    //     }
    // }

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
