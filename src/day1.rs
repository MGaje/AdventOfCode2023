#[path = "./util.rs"] mod util;

pub fn run() {
    let parsed_lines = util::read_file_to_lines("./inputs/day1-part1.txt");
    let mut parsed_values: Vec<i32> = Vec::new();

    for line in parsed_lines {
        let mut current_string_num = "".to_owned();
        let parsed_line = replace_strs_with_numerals(line);

        for c in parsed_line.chars() {
            if c.is_numeric() {
                current_string_num.push(c);
            }
        }

        let first_char = current_string_num.chars().nth(0).unwrap();
        let last_char = current_string_num.chars().nth_back(0).unwrap();
        let mut calib_num = "".to_owned();
        calib_num.push(first_char);
        calib_num.push(last_char);

        let number = calib_num.parse::<i32>().unwrap();
        parsed_values.push(number);

        println!("{number}");
    }

    let final_calib_num: i32 = parsed_values.iter().sum();
    println!("Final calibration: {final_calib_num}");
}

fn replace_strs_with_numerals(line: String) -> String {
    let mut parsed_str = line.clone();
    parsed_str = parsed_str.replace("one", "one1one");
    parsed_str = parsed_str.replace("two", "two2two");
    parsed_str = parsed_str.replace("three", "three3three");
    parsed_str = parsed_str.replace("four", "four4four");
    parsed_str = parsed_str.replace("five", "five5five");
    parsed_str = parsed_str.replace("six", "six6six");
    parsed_str = parsed_str.replace("seven", "seven7seven");
    parsed_str = parsed_str.replace("eight", "eight8eight");
    parsed_str = parsed_str.replace("nine", "nine9nine");

    return parsed_str;
}