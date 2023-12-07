use std::{iter::Peekable, str::Chars, fs::File, io::Read, collections::HashMap};

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut hash_map_card_id_to_num = HashMap::<i32, i32>::new();

    let iter_lines = contents.split_terminator('\n');
    for line in iter_lines {
        let mut vec_first_numbers = Vec::<i32>::new();
        let mut vec_second_numbers = Vec::<i32>::new();

        let mut peekable = line.chars().peekable();
        let _ = parse_word(&mut peekable);
        skip_whitespaces_and_special_characters(&mut peekable);
        let game_id = parse_number(&mut peekable);
        skip_whitespaces_and_special_characters(&mut peekable);

        while *peekable.peek().unwrap() != '|' {
            let number = parse_number(&mut peekable);
            skip_whitespaces_and_special_characters(&mut peekable);

            vec_first_numbers.push(number);
        }

        peekable.next();
        skip_whitespaces_and_special_characters(&mut peekable);

        while peekable.peek().is_some() {
            let number = parse_number(&mut peekable);
            skip_whitespaces_and_special_characters(&mut peekable);
            
            vec_second_numbers.push(number);
        }

        let mut total_intersects = 0;
        for num in vec_first_numbers {
            if vec_second_numbers.contains(&num) {
                total_intersects += 1;
            }            
        }

        hash_map_card_id_to_num.insert(game_id, total_intersects);
    }

    let mut cards_id_to_process = Vec::<i32>::new();
    for (k, _) in &hash_map_card_id_to_num {
        cards_id_to_process.push(*k)
    }

    let mut current_index = 0;
    while current_index < cards_id_to_process.len()  {
        let card_id = cards_id_to_process.get(current_index).unwrap();
        let val = *hash_map_card_id_to_num.get(&card_id).unwrap();

        for new_card_id in card_id+1..card_id+val+1  {
            if new_card_id > hash_map_card_id_to_num.len() as i32 {
                break;
            }

            cards_id_to_process.push(new_card_id);
        }

        current_index += 1
    }

    println!("part2: {}", cards_id_to_process.len());
}

fn part1() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_sum = 0;

    let iter_lines = contents.split_terminator('\n');
    for line in iter_lines {
        let mut vec_first_numbers = Vec::<i32>::new();
        let mut vec_second_numbers = Vec::<i32>::new();

        let mut peekable = line.chars().peekable();
        let _ = parse_word(&mut peekable);
        skip_whitespaces_and_special_characters(&mut peekable);
        let _ = parse_number(&mut peekable);
        skip_whitespaces_and_special_characters(&mut peekable);

        while *peekable.peek().unwrap() != '|' {
            let number = parse_number(&mut peekable);
            skip_whitespaces_and_special_characters(&mut peekable);

            vec_first_numbers.push(number);
        }

        peekable.next();
        skip_whitespaces_and_special_characters(&mut peekable);

        while peekable.peek().is_some() {
            let number = parse_number(&mut peekable);
            skip_whitespaces_and_special_characters(&mut peekable);
            
            vec_second_numbers.push(number);
        }

        let mut total_intersects = 0;
        for num in vec_first_numbers {
            if vec_second_numbers.contains(&num) {
                total_intersects += 1;
            }            
        }

        if total_intersects == 0 {
            continue;
        }

        let step: i32 = 2;
        total_sum += step.pow(total_intersects - 1);
    }

    println!("part1: {}", total_sum);
}

fn skip_whitespaces_and_special_characters(peekable: &mut Peekable::<Chars>) {
    while peekable.peek().is_some() && 
        (peekable.peek().unwrap().is_whitespace() ||
            *peekable.peek().unwrap() == ':'
        ) {
        peekable.next();
    }
}

fn parse_word(peekable: &mut Peekable::<Chars>) -> String {
    let mut word: Vec<char> = vec![];

    while peekable.peek().is_some() && peekable.peek().unwrap().is_alphabetic() {
        word.push(*peekable.peek().unwrap());
        peekable.next();
    }

    return word.into_iter().collect::<String>();
}

fn parse_number(peekable: &mut Peekable::<Chars>) -> i32 {
    let mut digit: Vec<char> = vec![];

    while peekable.peek().is_some() && peekable.peek().unwrap().is_digit(10) {
        digit.push(*peekable.peek().unwrap());
        peekable.next();
    }

    return digit.into_iter().collect::<String>().parse::<i32>().unwrap();
}