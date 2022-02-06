#[derive(Copy, Clone, Debug)]
pub struct BingoNum {
    number: usize,
    marked: bool,
}

impl BingoNum {
    pub fn new(number: usize, marked: bool) -> BingoNum {
        BingoNum { number, marked }
    }
}

pub struct Board {
    board_size: usize,
    numbers: [BingoNum; 25],
}

impl Board {
    pub fn new(numbers: [BingoNum; 25], board_size: usize) -> Board {
        Board {
            board_size,
            numbers,
        }
    }

    fn get_num(&self, x: usize, y: usize) -> &BingoNum {
        self.validate_index(x, y);
        let index = self.convert_to_index(x, y);
        &self.numbers[index]
    }

    pub fn mark_num(&mut self, number: usize) -> Option<(usize, usize)> {
        let mut counter = 0;
        while counter < self.numbers.len() && self.numbers[counter].number != number {
            counter += 1;
        }
        if counter < self.numbers.len() {
            self.numbers[counter].marked = true;
            return Some(self.convert_to_coord(counter));
        }
        None
    }

    pub fn check_bingo(&self, x: usize, y: usize) -> bool {
        self.validate_index(x, y);
        let mut counter = 0;
        while counter < self.board_size && self.get_num(x, counter).marked {
            counter += 1;
        }
        if counter == self.board_size {
            return true;
        } else {
            counter = 0;
            while counter < self.board_size && self.get_num(counter, y).marked {
                counter += 1;
            }
            if counter == self.board_size {
                return true;
            }
        }
        false
    }

    pub fn sum_non_marked_numbers(&self) -> usize {
        let mut sum = 0;
        for num in self.numbers {
            if !num.marked {
                sum += num.number;
            }
        }
        sum
    }

    fn convert_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.board_size
    }

    fn convert_to_coord(&self, index: usize) -> (usize, usize) {
        let y = index / 5;
        let x = index % 5;
        (x, y)
    }

    pub fn convert_to_bingo_numbers(number_array: &[usize; 25]) -> [BingoNum; 25] {
        let mut bingo_array = [BingoNum {
            number: 0,
            marked: false,
        }; 25];
        let mut counter = 0;

        for num in number_array {
            let bingo_num = BingoNum::new(*num, false);
            bingo_array[counter] = bingo_num;
            counter += 1;
        }

        bingo_array
    }

    fn validate_index(&self, x: usize, y: usize) {
        if x >= self.board_size || y >= self.board_size {
            panic!("Index out of range!");
        }
    }
}
