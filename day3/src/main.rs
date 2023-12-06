use std::{fs::File, io::Read};

fn main() {
    part1();
    part2();
}

fn part2() {

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
