#[path = "./util.rs"] mod util;

pub fn run() {
    let final_calibration_number = Day1::new().get_final_calibration_number();
    println!("Final calibration number: {}", final_calibration_number);
}

pub struct Day1 {
    calibration_number: i32
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            calibration_number: 0
        }
    }

    pub fn get_final_calibration_number(mut self) -> i32 {
        self.calibration_number = util::read_file_to_lines("./inputs/day1.txt")
            .iter()
            .map(|line| replace_strs_with_numerals(&line)
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>())
            .map(|mut digit_line| {
                    let len = digit_line.len();
                    if len == 1 {
                        return format!("{}{}", digit_line, digit_line);
                    }

                    // String was larger than 1 character so nuke the middle of the string.
                    digit_line.replace_range(1..(len - 1), "");
                    digit_line
                })
            .map(|digits| digits.parse::<i32>().unwrap())
            .sum();

        return self.calibration_number;
    }
}

///
/// This replaces all textual representations of numbers with a special output.
/// This output begins with the first letter of the number word (for example: "one" would be "o").
/// Then the middle is the numeral (for example: "one" would be "1").
/// Then it ends with the last letter of the word number (for example: "one" would be "e").
/// 
/// This is done to combat the overlapping (evil) number outputs like "eightwo" or "oneight" while also introducing the digits the parsing
/// in day1 can utilize.
/// 
fn replace_strs_with_numerals(line: &String) -> String {
    return line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}