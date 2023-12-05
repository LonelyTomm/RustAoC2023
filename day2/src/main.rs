use std::{fs::File, io::Read, iter::Peekable, str::Chars, collections::HashMap};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut max_numbers = HashMap::new();
    max_numbers.insert(String::from("blue"), 14);
    max_numbers.insert(String::from("red"), 12);
    max_numbers.insert(String::from("green"), 13);

    let mut sum_of_game_ids = 0;

    let iter_lines = contents.split_terminator('\n');

    for line in iter_lines {
        let mut peekable = line.chars().peekable();
        sum_of_game_ids += check_game(&mut peekable, &max_numbers);
    }

    println!("part1: {}", sum_of_game_ids);
}

fn part2() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut sum_of_powers = 0;
    let iter_lines = contents.split_terminator('\n');
    for line in iter_lines {
        let mut peekable = line.chars().peekable();
        sum_of_powers += check_game_power(&mut peekable)
    }

    println!("part2: {}", sum_of_powers);
}

fn check_game_power(peekable: &mut Peekable::<Chars>) -> i32 {
    let _ = parse_word(peekable);
    skip_whitespaces_and_special_characters(peekable);
    let _ = parse_number(peekable);

    let mut red = 1;
    let mut blue = 1;
    let mut green = 1;

    while peekable.peek().is_some() {
        skip_whitespaces_and_special_characters(peekable);
        let num_dices = parse_number(peekable);
        skip_whitespaces_and_special_characters(peekable);
        let color_name = parse_word(peekable);
        
        match color_name.as_str() {
            "red" => if num_dices > red {
                red = num_dices;
            },
            "blue" => if num_dices > blue {
                blue = num_dices;
            },
            "green" => if num_dices > green {
                green = num_dices;
            },
            _ => panic!("error")
        }
    }

    return red * blue * green;
}

fn check_game(peekable: &mut Peekable::<Chars>, max_numbers: &HashMap<String, i32>) -> i32 {
    let _ = parse_word(peekable);
    skip_whitespaces_and_special_characters(peekable);
    let game_id = parse_number(peekable);

    while peekable.peek().is_some() {
        skip_whitespaces_and_special_characters(peekable);
        let num_dices = parse_number(peekable);
        skip_whitespaces_and_special_characters(peekable);
        let color_name = parse_word(peekable);

        if num_dices > *max_numbers.get(&color_name).unwrap() {
            return 0;
        }
    }

    return game_id;
}

fn skip_whitespaces_and_special_characters(peekable: &mut Peekable::<Chars>) {
    while peekable.peek().is_some() && 
        (peekable.peek().unwrap().is_whitespace() ||
            *peekable.peek().unwrap() == ',' ||
            *peekable.peek().unwrap() == ':' ||
            *peekable.peek().unwrap() == ';'
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
