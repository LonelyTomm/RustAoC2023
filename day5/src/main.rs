use std::{fs::File, io::Read, iter::Peekable, str::Chars, collections::HashMap};

fn main() {
    part1();
    part2();
}

struct Range {
    min_value: i64,
    max_value: i64,
}

fn part2() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    
    let lines = contents.split_terminator('\n');

    let mut map_name_to_vals = HashMap::<String, Vec<Vec<i64>>>::new();
    let mut ref_to_curr_hash_map_vec = &mut Vec::<Vec::<i64>>::new();
    let mut seeds_vec = Vec::<i64>::new();
    for line in lines  {
        let mut peekable = line.chars().peekable();

        if peekable.peek().is_none() {
            continue;
        }
        
        if peekable.peek().is_some() && peekable.peek().unwrap().is_digit(10) {
            let mut number_vec = Vec::<i64>::new();
            number_vec.push(read_number(&mut peekable));
            skip_whitespaces_and_special_chars(&mut peekable);

            number_vec.push(read_number(&mut peekable));
            skip_whitespaces_and_special_chars(&mut peekable);

            number_vec.push(read_number(&mut peekable));
            ref_to_curr_hash_map_vec.push(number_vec);
            continue;
        }

        let current_word = read_word(&mut peekable);
        if current_word == "seeds" {
            while peekable.peek().is_some() {
                skip_whitespaces_and_special_chars(&mut peekable);
                seeds_vec.push(read_number(&mut peekable));  
            }
            
            continue;
        }

        map_name_to_vals.insert(current_word.clone(), Vec::<Vec<i64>>::new());
        ref_to_curr_hash_map_vec = map_name_to_vals.get_mut(&current_word).unwrap();
    }
    
    let mut min_location: i64 = -1;
    let mut i = 0;
    while i < seeds_vec.len() {
        let mut seed_range = Range {
            min_value: seeds_vec[i],
            max_value: seeds_vec[i] + seeds_vec[i+1],
        };

        let mut vec_range = calclulate_range_from_map(&mut seed_range, &map_name_to_vals, "seed-to-soil");
        for name in ["soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"] {
            let mut new_vec_range = Vec::<Range>::new();

            for range in vec_range {
                let mut new_range = range;
                new_vec_range.extend(calclulate_range_from_map(&mut new_range, &map_name_to_vals, name));
            }

            vec_range = new_vec_range;
        }
        
        for range in vec_range {
            if min_location == -1 || range.min_value < min_location {
                min_location = range.min_value;
            }
        }

        i += 2;
    }

    println!("part2: {}", min_location);
}

fn calclulate_range_from_map(source_range: &mut Range, hash_map: &HashMap::<String, Vec<Vec<i64>>>, map_name: &str) -> Vec<Range> {
    let mut vec_rang = Vec::<Range>::new();
    let map = &hash_map[map_name];

    for map_entry in map  {
        let dest_range_start = map_entry[0];
        let source_range_start = map_entry[1];
        let range_length = map_entry[2];

        if source_range.min_value >= source_range_start && source_range.min_value < source_range_start + range_length {
            let min_value = (source_range.min_value - source_range_start) + dest_range_start;
            let max_value: i64;
            
            if source_range.max_value <= source_range_start + range_length {
                max_value = (source_range.max_value - source_range_start) + dest_range_start;
                vec_rang.push(Range { min_value: min_value, max_value: max_value });
                return vec_rang;
            }

            max_value = dest_range_start + range_length;
            vec_rang.push(Range { min_value: min_value, max_value: max_value });
            source_range.min_value  = source_range_start + range_length;
            vec_rang.extend(calclulate_range_from_map(source_range, hash_map, map_name));
            return vec_rang;
        }
    }
    
    vec_rang.push(Range { min_value: source_range.min_value, max_value: source_range.max_value });
    return vec_rang;
}

fn part1() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    
    let lines = contents.split_terminator('\n');

    let mut map_name_to_vals = HashMap::<String, Vec<Vec<i64>>>::new();
    let mut ref_to_curr_hash_map_vec = &mut Vec::<Vec::<i64>>::new();
    let mut seeds_vec = Vec::<i64>::new();
    for line in lines  {
        let mut peekable = line.chars().peekable();

        if peekable.peek().is_none() {
            continue;
        }
        
        if peekable.peek().is_some() && peekable.peek().unwrap().is_digit(10) {
            let mut number_vec = Vec::<i64>::new();
            number_vec.push(read_number(&mut peekable));
            skip_whitespaces_and_special_chars(&mut peekable);

            number_vec.push(read_number(&mut peekable));
            skip_whitespaces_and_special_chars(&mut peekable);

            number_vec.push(read_number(&mut peekable));
            ref_to_curr_hash_map_vec.push(number_vec);
            continue;
        }

        let current_word = read_word(&mut peekable);
        if current_word == "seeds" {
            while peekable.peek().is_some() {
                skip_whitespaces_and_special_chars(&mut peekable);
                seeds_vec.push(read_number(&mut peekable));  
            }
            
            continue;
        }

        map_name_to_vals.insert(current_word.clone(), Vec::<Vec<i64>>::new());
        ref_to_curr_hash_map_vec = map_name_to_vals.get_mut(&current_word).unwrap();
    }

    let mut min_location: i64 = -1;
    for seed in seeds_vec {
        let soil = find_by_source_in_map(seed, &map_name_to_vals["seed-to-soil"]);
        let fertilizer = find_by_source_in_map(soil, &map_name_to_vals["soil-to-fertilizer"]);
        let water = find_by_source_in_map(fertilizer, &map_name_to_vals["fertilizer-to-water"]);
        let light = find_by_source_in_map(water, &map_name_to_vals["water-to-light"]);
        let temp = find_by_source_in_map(light, &map_name_to_vals["light-to-temperature"]);
        let hum = find_by_source_in_map(temp, &map_name_to_vals["temperature-to-humidity"]);
        let location = find_by_source_in_map(hum, &map_name_to_vals["humidity-to-location"]);

        if min_location == -1 || location < min_location {
            min_location = location;
        }
    }

    println!("part1: {}", min_location);
}

fn find_by_source_in_map(source_val: i64, map: &Vec::<Vec::<i64>>) -> i64 {
    for map_entry in map  {
        if source_val >= map_entry[1] && source_val <= map_entry[1] + map_entry[2] {
            return (source_val - map_entry[1]) + map_entry[0];
        }
    }

    return source_val;
}

fn read_word(peekable: &mut Peekable::<Chars>) -> String {
    let mut char_vec: Vec<char> = Vec::<char>::new();
    while peekable.peek().is_some() && (peekable.peek().unwrap().is_alphabetic() || *peekable.peek().unwrap() == '-') {
        char_vec.push(*peekable.peek().unwrap());
        peekable.next();
    }

    return char_vec.iter().collect::<String>();
}

fn skip_whitespaces_and_special_chars(peekable: &mut Peekable::<Chars>) {
    while peekable.peek().is_some() && (peekable.peek().unwrap().is_whitespace() || *peekable.peek().unwrap() == ':') {
        peekable.next();
    }
}

fn read_number(peekable: &mut Peekable::<Chars>) -> i64 {
    let mut num_vec: Vec<char> = Vec::<char>::new();
    while peekable.peek().is_some() && peekable.peek().unwrap().is_digit(10) {
        num_vec.push(*peekable.peek().unwrap());
        peekable.next();
    }

    return num_vec.iter().collect::<String>().parse::<i64>().unwrap();
}
