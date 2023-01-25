use std::{fmt::Display, vec};

#[derive(Clone)]
pub enum FoldInstruction {
    X(usize),
    Y(usize),
}

pub enum NumberOfFolds {
    All,
    SpecificNumber(usize),
}

pub struct TransparentPaper {
    paper: Vec<Vec<bool>>,
    fold_instructions: Vec<FoldInstruction>,
    max_size: (usize, usize),
}

impl TransparentPaper {
    pub fn new(x: usize, y: usize, fold_instructions: Vec<FoldInstruction>) -> TransparentPaper {
        TransparentPaper {
            paper: vec![vec![false; y]; x],
            max_size: (x, y),
            fold_instructions,
        }
    }

    pub fn fold_according_instructions(&mut self, num_of_steps: NumberOfFolds) {
        match num_of_steps {
            NumberOfFolds::All => {
                for instruction in self.fold_instructions.clone() {
                    self.fold(instruction);
                }
            }
            NumberOfFolds::SpecificNumber(num_of_folds) => {
                let mut counter = 0;
                for instruction in self.fold_instructions.clone() {
                    if counter == num_of_folds {
                        break;
                    }
                    self.fold(instruction);
                    counter += 1;
                }
                if counter < num_of_folds {
                    println!("Not enough instruction!");
                }
            }
        }
    }

    pub fn fold(&mut self, fold_instruction: FoldInstruction) {
        match fold_instruction {
            FoldInstruction::X(x) => {
                for x_index in 0..self.paper.len() {
                    for y_index in 0..self.paper[0].len() {
                        if y_index > x {
                            let offset = x as isize - (y_index as isize - x as isize);
                            if offset >= 0 && self.paper[x_index][y_index] {
                                self.paper[x_index][offset as usize] = true;
                            }
                        }
                    }
                }
                self.max_size.1 /= 2;
            }
            FoldInstruction::Y(y) => {
                for x_index in 0..self.paper.len() {
                    if x_index > y {
                        for y_index in 0..self.paper[0].len() {
                            let offset = y as isize - (x_index as isize - y as isize);
                            if offset >= 0 && self.paper[x_index][y_index] {
                                self.paper[offset as usize][y_index] = true;
                            }
                        }
                    }
                }

                self.max_size.0 /= 2;
            }
        }
    }

    pub fn add_dot(&mut self, x: usize, y: usize) {
        self.paper[y][x] = true;
    }

    pub fn count_dots(&self) -> usize {
        let mut counter = 0;
        let mut x_counter = 0;
        while x_counter < self.max_size.0 {
            let mut y_counter = 0;
            while y_counter < self.max_size.1 {
                if self.paper[x_counter][y_counter] {
                    counter += 1;
                };
                y_counter += 1;
            }
            x_counter += 1;
        }
        counter
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_str = String::new();
        let mut x_counter = 0;
        while x_counter < self.max_size.0 {
            let mut y_counter = 0;
            while y_counter < self.max_size.1 {
                output_str.push(if self.paper[x_counter][y_counter] {
                    '#'
                } else {
                    '.'
                });
                y_counter += 1;
            }
            output_str.push('\n');
            x_counter += 1;
        }

        write!(f, "{}", output_str)
    }
}
