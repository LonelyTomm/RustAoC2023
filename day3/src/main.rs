use std::{fs::File, io::Read, collections::HashMap};

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map_start_idx_to_numbers = HashMap::<String, Vec<i32>>::new();

    let lines = contents.split_terminator('\n').collect::<Vec<&str>>();

    let mut total_sum: i64 = 0;

    let mut number_vec = Vec::<char>::new();
    let mut adjacent_stars = Vec::<String>::new();
    for i in 0..lines.len()  {

        for j in 0..lines[i].len()  {
            let chr = lines[i].chars().collect::<Vec<char>>()[j];
            
            if chr.is_digit(10) {
                number_vec.push(chr);
                let vec_adjacent_stars_idxs = find_adjacent_stars(i, j, &lines);
                for idx in &vec_adjacent_stars_idxs {
                    if !adjacent_stars.contains(idx) {
                        adjacent_stars.push(idx.to_string());
                    }
                }
            }

            if !chr.is_digit(10) && number_vec.len() > 0 {
                if adjacent_stars.len() > 0 { 
                    for star in &adjacent_stars {
                        if map_start_idx_to_numbers.get(star).is_none() {
                            map_start_idx_to_numbers.insert(star.to_string(), Vec::<i32>::new());
                        }

                        map_start_idx_to_numbers.get_mut(star).unwrap().push(number_vec.clone().into_iter().collect::<String>().parse::<i32>().unwrap());
                    }
                }

                adjacent_stars.clear();
                number_vec.clear();
            }
        }
    }

    for (_, vec_numbers) in map_start_idx_to_numbers {
        if vec_numbers.len() == 2 {
            total_sum += vec_numbers[0] as i64 * vec_numbers[1] as i64;
        }
    }

    println!("part2: {}", total_sum);
}

fn part1() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.split_terminator('\n').collect::<Vec<&str>>();

    let mut total_sum = 0;

    let mut number_vec = Vec::<char>::new();
    let mut is_part_number = false;
    for i in 0..lines.len()  {

        for j in 0..lines[i].len()  {
            let chr = lines[i].chars().collect::<Vec<char>>()[j];
            
            if chr.is_digit(10) {
                number_vec.push(chr);
                if !is_part_number {
                    is_part_number = check_adjacency(i, j, &lines);
                }
            }

            if !chr.is_digit(10) && number_vec.len() > 0 {
                if is_part_number { 
                    total_sum += number_vec.clone().into_iter().collect::<String>().parse::<i32>().unwrap();
                }

                is_part_number = false;
                number_vec.clear();
            }
        }
    }

    println!("part1: {}", total_sum);      
}

fn check_adjacency(i: usize,j: usize, lines: &Vec<&str>) -> bool {
    let ovi = if i == 0 { i } else { i - 1 };
    let ovj = if j == 0 { j } else { j - 1 };
    for newi in ovi..i + 2  {
        if lines.get(newi).is_none() {
            continue;
        }

        let line_vec = lines[newi].chars().collect::<Vec<char>>();
        for newj in ovj..j + 2 {
            if line_vec.get(newj).is_none() {
                continue;
            }

            if !line_vec[newj].is_digit(10) && line_vec[newj] != '.' {
                return true;
            }
        }
    }

    return false;
}

fn find_adjacent_stars(i: usize,j: usize, lines: &Vec<&str>) -> Vec<String> {
    let mut vec_star_indexes = Vec::<String>::new();

    let ovi = if i == 0 { i } else { i - 1 };
    let ovj = if j == 0 { j } else { j - 1 };
    for newi in ovi..i + 2  {
        if lines.get(newi).is_none() {
            continue;
        }

        let line_vec = lines[newi].chars().collect::<Vec<char>>();
        for newj in ovj..j + 2 {
            if line_vec.get(newj).is_none() {
                continue;
            }

            if line_vec[newj] == '*' {
                let idx_string = newi.to_string() + ";" + &newj.to_string();
                vec_star_indexes.push(idx_string);
            }
        }
    }
    return vec_star_indexes;
}
