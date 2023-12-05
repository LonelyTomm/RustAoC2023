use std::{fs::File, io::Read};


fn main() {
    part1();
    part2();
}

fn part1() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_calibration_sum = 0;

    let iter_lines = contents.split_whitespace();
    for line in iter_lines {
        let mut first_digit = -1;
        let mut last_digit = -1;

        for char in line.chars()  {
            if char.is_numeric() {
                if first_digit < 0 {
                    first_digit = char.to_digit(10).unwrap() as i32;
                    last_digit = first_digit;
                } else {
                    last_digit = char.to_digit(10).unwrap() as i32;
                }
            } 
        }
        
        total_calibration_sum += (first_digit * 10) + last_digit;
    }  

    println!("Part 1: {}", total_calibration_sum);
}

fn part2() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_calibration_sum = 0;

    let iter_lines = contents.split_whitespace();
    for line in iter_lines {
        let mut first_digit = -1;
        let mut last_digit = -1;

        let char_vec = line.chars().collect::<Vec<char>>();

        for mut indx in 0..char_vec.len() {
            let char = char_vec.get(indx).unwrap();

            let char_digit_value;
            if char.is_digit(10) {
               char_digit_value = char.to_digit(10).unwrap() as i32;
            } else {
                char_digit_value = try_parse_number(&char_vec, &mut indx)
            }

            if char_digit_value < 0 {
                continue;
            }

            if first_digit < 0 {
                first_digit = char_digit_value;
                last_digit = first_digit;
            } else {
                last_digit = char_digit_value;
            }
        }

        total_calibration_sum += (first_digit * 10) + last_digit;
    }  

    println!("Part 2: {}", total_calibration_sum);
}

const DIGIT_WORDS_ARRAY: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const POSSIBLE_SIZE: [i32; 3] = [3, 4 ,5];

fn try_parse_number(char_vec: &Vec<char>, indx: &mut usize) -> i32 {
    for size in POSSIBLE_SIZE  {
        if *indx as i32 + size > char_vec.len() as i32 {
            break;
        }

        let word: &str = &char_vec[(*indx as usize)..(*indx as usize + size as usize)].into_iter().collect::<String>();
        if DIGIT_WORDS_ARRAY.contains(&word) {
            let x = match word {
                "one" => 1,
                "two" => 2,
                "three" => 3, 
                "four" => 4, 
                "five" => 5, 
                "six" => 6, 
                "seven" => 7, 
                "eight" => 8, 
                "nine" => 9,
                _ => -1 
            };

            *indx += size as usize - 1;
            return x;
        }
    }

    return -1;
}
