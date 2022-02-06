fn main() {
    let data_array = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    let corrupted_lines = get_corrupted(data_array);

    for lines in &corrupted_lines {
        println!("{}", *lines);
    }

    let point = point_corrupted(corrupted_lines);

    println!("The total syntax error score is: {}", point);
}

fn get_corrupted(data_array: Vec<&str>) -> Vec<&str> {
    let mut stack: Vec<char> = vec![];
    let mut ret: Vec<&str> = vec![];

    for line in data_array {
        let mut is_corrupted = false;

        for brackets in line.chars() {
            match brackets {
                '[' | '(' | '{' | '<' => stack.push(brackets),
                ']' | ')' | '}' | '>' => {
                    let closing_bracket = match brackets {
                        ']' => '[',
                        ')' => '(',
                        '>' => '<',
                        '}' => '{',
                        _ => panic!("Invalid!"),
                    };

                    if stack.pop().unwrap() != closing_bracket {
                        is_corrupted = true;
                        break;
                    }
                }
                _ => panic!("Invalid"),
            };
        }

        if is_corrupted {
            ret.push(line);
        }
    }

    ret
}

fn point_corrupted(data_array: Vec<&str>) -> isize {
    let mut stack: Vec<char> = vec![];
    let mut point = 0;

    for line in data_array {
        for brackets in line.chars() {
            match brackets {
                '[' | '(' | '{' | '<' => stack.push(brackets),
                ']' | ')' | '}' | '>' => {
                    let closing_bracket = match brackets {
                        ']' => '[',
                        ')' => '(',
                        '>' => '<',
                        '}' => '{',
                        _ => panic!("Invalid!"),
                    };

                    if stack.pop().unwrap() != closing_bracket {
                        point += match closing_bracket {
                            '(' => 3,
                            '[' => 57,
                            '{' => 1197,
                            '<' => 25137,
                            _ => panic!("Invalid"),
                        };

                        break;
                    }
                }
                _ => panic!("Invalid"),
            };
        }
    }
    point
}
