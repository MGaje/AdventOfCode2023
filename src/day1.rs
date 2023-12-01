#[path = "./util.rs"] mod util;

pub fn run() {
    let calibration_number: i32 = util::read_file_to_lines("./inputs/day1.txt")
        .iter()
        .map(|line|  replace_strs_with_numerals(&line)
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>())
        .map(|mut digit_line| {
                let len = digit_line.len();
                if len == 1 {
                    return format!("{}{}", digit_line, digit_line);
                }

                digit_line.replace_range(1..(len - 1), "");
                digit_line
            })
        .map(|digits| digits.parse::<i32>().unwrap())
        .sum();

    println!("Final calibration number: {}", calibration_number);
}

fn replace_strs_with_numerals(line: &String) -> String {
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