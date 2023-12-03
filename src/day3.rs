#[path = "./util.rs"] mod util;

use std::{collections::HashSet};
use regex::Regex;

pub fn run() {
    let day3 = Day3::new();
    //day3.part_1();
    day3.part_2();
}

pub struct Day3 {
    // Empty.
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3{}
    }

    pub fn part_1(self) {
        let output = util::read_file_to_lines("./inputs/day3.txt");

        let mut i = 0;
        let mut prev_line: String = String::from("");
        let mut current_line: String = String::from("");
        let mut next_line: String = String::from("");

        let mut all_part_numbers: Vec<String> = Vec::new();

        while i < output.len() {
            prev_line = String::from(&current_line);
            current_line = String::from(&output[i]);

            if (i + 1) < output.len() {
                next_line = String::from(&output[i + 1]);
            }
            else {
                next_line = String::from("");
            }

            let mut part_numbers_prev = Day3::_find_part_numbers_in_lines(&current_line, &prev_line);
            let mut part_numbers_current = Day3::_find_part_numbers_in_lines(&current_line, &current_line);
            let mut part_numbers_next = Day3::_find_part_numbers_in_lines(&current_line, &next_line);

            let mut all_part_numbers_this_iter: Vec<(usize, String)> = Vec::new();
            all_part_numbers_this_iter.extend(part_numbers_prev.iter().cloned());
            all_part_numbers_this_iter.extend(part_numbers_current.iter().cloned());
            all_part_numbers_this_iter.extend(part_numbers_next.iter().cloned());


            all_part_numbers_this_iter.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
            all_part_numbers.extend(all_part_numbers_this_iter.iter().cloned().map(|x|x.1));

            i += 1;
        }

        let part_number_sum: i32 = all_part_numbers
            .iter()
            .map(|x|x.parse::<i32>().unwrap())
            .sum();

        println!("All Part Numbers: {}", all_part_numbers.join(","));
        println!("Part number sum: {}", part_number_sum);

    }

    pub fn part_2(self) {
        let output = util::read_file_to_lines("./inputs/day3.txt");

        let mut i = 0;
        let mut prev_line: String = String::from("");
        let mut current_line: String = String::from("");
        let mut next_line: String = String::from("");

        let mut gear_ratios: Vec<i32> = Vec::new();

        while i < output.len() {
            prev_line = String::from(&current_line);
            current_line = String::from(&output[i]);

            if (i + 1) < output.len() {
                next_line = String::from(&output[i + 1]);
            }
            else {
                next_line = String::from("");
            }

            let gear_number_sets = Day3::_find_gears(&current_line, &prev_line, &next_line);

            println!("");
            println!("--Line {}--", i + 1);

            for set in gear_number_sets {
                print!("[ {} ] -> ", set.iter().map(|x|x.1.to_string()).collect::<Vec<String>>().join(","));
                
                let mut p = 1;
                for number in set {
                    p *= number.1.parse::<i32>().unwrap();
                }

                print!("{}", p);
                println!("");

                gear_ratios.push(p);
            }

            println!("--END Line {}--", i + 1);

            println!("");

            i += 1;
        }

        println!("");
        let total_sum: i32 = gear_ratios.iter().sum();
        println!("Sum of gear ratios: {}", total_sum);
        println!("");

    }

    fn _find_numbers(line: &String) -> Vec<(usize, String)> {
        let number_re = Regex::new(r"(\d+)+").unwrap();

        return number_re
            .find_iter(&line)
            .filter_map(|number| Some((number.start(), number.as_str().parse().ok().unwrap())))
            .collect::<Vec<(usize, String)>>();
    }

    fn _find_gear_symbol_positions(haystack: &String) -> Vec<usize> {
        let mut indices = Vec::new();

        for (index, c) in haystack.char_indices() {
            if c == '*' {
                indices.push(index);
            }
        }

        return indices;
    }

    fn _find_symbol_positions(haystack: &String) -> Vec<usize> {
        let mut indices = Vec::new();

        for (index, c) in haystack.char_indices() {
            if !c.is_alphanumeric() && c != '.' {
                indices.push(index);
            }
        }

        return indices;
    }

    fn _find_gears(current_line: &String, prev_line: &String, next_line: &String) -> Vec<Vec<(usize, String)>> {
        let gear_symbol_positions = Day3::_find_gear_symbol_positions(current_line);
        if gear_symbol_positions.len() == 0 {
            return vec![vec![(0, String::from("0")), (0, String::from("0"))]];
        }

        let mut gear_number_sets: Vec<Vec<(usize, String)>> = Vec::new();

        for gear_symbol_position in gear_symbol_positions {
            let mut gear_numbers: Vec<(usize, String)> = Vec::new();

            // Previous line.
            if prev_line.len() > 0 {
                let prev_numbers = Day3::_find_numbers(&prev_line);
                
                for prev_number in prev_numbers {
                    let number_string_end_pos = prev_number.0 + (prev_number.1.len() - 1);
                    let start_check = if prev_number.0 > 0 { prev_number.0 - 1} else { 0 };
                    let end_check = number_string_end_pos + 1;

                    if gear_symbol_position >= start_check && gear_symbol_position <= end_check {
                        gear_numbers.push(prev_number);
                    }
                }
            }

            // Current line.
            let current_numbers = Day3::_find_numbers(&current_line);
            for current_number in current_numbers {
                let number_string_end_pos = current_number.0 + (current_number.1.len() - 1);
                let start_check = if current_number.0 > 0 { current_number.0 - 1} else { 0 };
                let end_check = number_string_end_pos + 1;

                if gear_symbol_position >= start_check && gear_symbol_position <= end_check {
                    gear_numbers.push(current_number);
                }
            }

            // Next line.
            if next_line.len() > 0 {
                let next_numbers = Day3::_find_numbers(&next_line);
                for next_number in next_numbers {
                    let number_string_end_pos = next_number.0 + (next_number.1.len() - 1);
                    let start_check = if next_number.0 > 0 { next_number.0 - 1} else { 0 };
                    let end_check = number_string_end_pos + 1;

                    if gear_symbol_position >= start_check && gear_symbol_position <= end_check {
                        gear_numbers.push(next_number);
                    }
                }
            }

            if gear_numbers.len() == 2 {
                gear_number_sets.push(gear_numbers);
            }
        }

        return gear_number_sets;

    }

    fn _combine_part_number_vectors(v: &Vec<String>, v2: &Vec<String>, remove_dupes: bool) -> Vec<String> {
        let mut combined = v.clone();
        combined.extend(v2.clone());

        if remove_dupes {
            return combined
                .into_iter()
                .collect::<HashSet<String>>()
                .into_iter()
                .collect();
        }

        return combined;
    }

    fn _find_part_numbers_in_lines(target_line: &String, compare_line: &String) -> Vec<(usize, String)> {
        let symbol_positions = Day3::_find_symbol_positions(&compare_line);
        let numbers = Day3::_find_numbers(&target_line);
        let mut part_numbers = Vec::new();

        for number in numbers.iter() {

            let number_string_end_pos = number.0 + (number.1.len() - 1);
            let start_check = if number.0 > 0 { number.0 - 1} else { 0 };
            let end_check = number_string_end_pos + 1;

            for symbol_pos in symbol_positions.iter() {
                if symbol_pos >= &start_check && symbol_pos <= &end_check {
                    part_numbers.push(number.clone());
                }
            }
        }

        return part_numbers;
    }
}