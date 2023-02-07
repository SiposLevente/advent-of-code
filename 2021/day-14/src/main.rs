use std::{collections::HashMap, env::args, fs::read_to_string, cmp::Reverse};

fn main() {
    let input_data = read_input("puzzle.txt");

    let polymerized_template = polymerize(
        input_data.0,
        &input_data.1,
        args()
            .nth(1)
            .expect("Invalid argumentum")
            .parse::<usize>()
            .expect("Please input a number!"),
    );

    let counter_items = get_element_occurance(&polymerized_template);
    println!("{:?}", counter_items);

    println!(
        "Final result: {} - {} = {}",
        counter_items[0].1,
        counter_items[counter_items.len() - 1].1,
        counter_items[0].1 - counter_items[counter_items.len() - 1].1
    );
}

fn get_element_occurance(template: &str) -> Vec<(char, usize)> {
    let mut map_char_counter: HashMap<char, usize> = HashMap::new();
    for char in template.chars() {
        if let Some(value) = map_char_counter.get_mut(&char) {
            *value += 1;
        } else {
            map_char_counter.insert(char, 1);
        }
    }

    let mut ret_vector = map_char_counter
        .into_iter()
        .map(|(key, value)| (key, value))
        .collect::<Vec<(char, usize)>>();

        
    ret_vector.sort_by_key(|x| Reverse(x.1));
    ret_vector
}

fn polymerize(template: String, rules: &HashMap<(char, char), char>, steps: usize) -> String {
    let mut final_template = template;
    let mut temp_template = String::new();

    for step in 0..steps {
        println!("Working on step: {step}");
        let pairs = create_pairs(&final_template);
        let pair_size = pairs.len() - 1;
        for (counter, pair) in pairs.into_iter().enumerate() {
            if let Some(product) = rules.get(&pair) {
                if counter % 2 == 0 {
                    temp_template.push_str(format!("{}{}{}", pair.0, product, pair.1).as_str());
                } else {
                    temp_template.push(*product);
                }

                if counter == pair_size && counter % 2 == 1 {
                    temp_template.push(pair.1);
                }
            } else {
                temp_template.push_str(format!("{}{}", pair.0, pair.1).as_str());
            }
        }
        final_template = temp_template.clone();
        temp_template.clear();
    }
    final_template
}

fn create_pairs(template: &str) -> Vec<(char, char)> {
    let template_chars: Vec<char> = template.chars().collect();
    let mut pairs = vec![];
    for index in 0..template_chars.len() - 1 {
        pairs.push((template_chars[index], template_chars[index + 1]));
    }
    pairs
}

fn read_input(puzzle_file: &str) -> (String, HashMap<(char, char), char>) {
    let mut polymer_template = String::new();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    if let Ok(data) = read_to_string(puzzle_file) {
        let clean_data = data.replace('\r', "");
        for data_line in clean_data.split('\n') {
            if data_line.contains('-') {
                let data_in_line: Vec<&str> = data_line.split(" -> ").collect();
                let product: Vec<char> = data_in_line[1].chars().collect();
                let elements: Vec<char> = data_in_line[0].chars().collect();
                rules.insert((elements[0], elements[1]), product[0]);
            } else if !data_line.is_empty() {
                polymer_template = data_line.to_owned();
            }
        }
    }

    (polymer_template, rules)
}
