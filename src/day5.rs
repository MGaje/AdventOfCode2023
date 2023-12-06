#[path = "./util.rs"] mod util;

use std::fs::read_to_string;
use regex::Regex;
use std::ops::Range;

pub fn run() {
    let day5 = Day5::new();
    //day5.part_1();
    day5.part_2();
}

pub struct Day5 {
    // Empty.
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {}
    }

    pub fn part_1(self) {
        let data = read_to_string("./inputs/day5.txt").unwrap();
        let mut seed_list = Day5::_parse_seed_list(&data);

        let seed_to_soil_data = Day5::_parse_seed_to_soil_map(&data);
        let soil_to_fert_data = Day5::_parse_soil_to_fertilizer_map(&data);
        let fert_to_water_data = Day5::_parse_fertilizer_to_water_map(&data);
        let water_to_light_data = Day5::_parse_water_to_light_map(&data);
        let light_to_temperature_data = Day5::_parse_light_to_temperature_map(&data);
        let temperature_to_humidity_data = Day5::_parse_temperature_to_humidity_map(&data);
        let humidity_to_location_data = Day5::_parse_humidity_to_location_map(&data);

        let mut locations: Vec<u64> = Vec::new();
        for seed in seed_list {

            let soil = Day5::_convert(&seed_to_soil_data, seed);
            let fert = Day5::_convert(&soil_to_fert_data, soil);
            let water = Day5::_convert(&fert_to_water_data, fert);
            let light = Day5::_convert(&water_to_light_data, water);
            let temp = Day5::_convert(&light_to_temperature_data, light);
            let humidity = Day5::_convert(&temperature_to_humidity_data, temp);
            let location = Day5::_convert(&humidity_to_location_data, humidity);

            locations.push(location);

        }

        let min_location = locations.iter().min().unwrap();
        println!("Minimum location: {}", min_location);
        
    }

    pub fn part_2(self) {
        let data = read_to_string("./inputs/day5.txt").unwrap();
        let mut seed_list = Day5::_parse_seed_list(&data);
        let chunks: Vec<&[u64]> = seed_list.chunks(2).collect();

        let mut full_seed_list: Vec<u64> = Vec::new();
        for c in chunks {
            let start = c[0];
            let end = start + c[1];
            let mut v = Vec::from_iter(start..end);

            full_seed_list.append(&mut v);
        }

        full_seed_list.sort();

        let seed_to_soil_data = Day5::_parse_seed_to_soil_map(&data);
        let soil_to_fert_data = Day5::_parse_soil_to_fertilizer_map(&data);
        let fert_to_water_data = Day5::_parse_fertilizer_to_water_map(&data);
        let water_to_light_data = Day5::_parse_water_to_light_map(&data);
        let light_to_temperature_data = Day5::_parse_light_to_temperature_map(&data);
        let temperature_to_humidity_data = Day5::_parse_temperature_to_humidity_map(&data);
        let humidity_to_location_data = Day5::_parse_humidity_to_location_map(&data);

        let mut locations: Vec<u64> = Vec::new();
        for seed in full_seed_list {

            let soil = Day5::_convert(&seed_to_soil_data, seed);
            let fert = Day5::_convert(&soil_to_fert_data, soil);
            let water = Day5::_convert(&fert_to_water_data, fert);
            let light = Day5::_convert(&water_to_light_data, water);
            let temp = Day5::_convert(&light_to_temperature_data, light);
            let humidity = Day5::_convert(&temperature_to_humidity_data, temp);
            let location = Day5::_convert(&humidity_to_location_data, humidity);

            locations.push(location);

        }

        let min_location = locations.iter().min().unwrap();
        println!("Minimum location: {}", min_location);
    }

    fn _parse_seed_list(input: &String) -> Vec<u64> {
        let pattern = Regex::new(r"seeds: (\d+\s)+").unwrap();
        let Some(captures) = pattern.captures(input) else { return Vec::new() };

        let seed_list_parts = captures[0]
            .split(":")
            .map(|x|x.trim().to_string())
            .collect::<Vec<String>>();

        return seed_list_parts[1]
            .split_ascii_whitespace()
            .map(|x|x.to_string().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
    }

    fn _convert(data: &Vec<Vec<u64>>, search: u64) -> u64 {
        for set in data {
            if search >= set[1] && search <= &set[1] + &set[2] {
                let delta = search - set[1];
                return set[0] + delta;
            }
        }

        return search;
    }

    fn _parse_seed_to_soil_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("seed-to-soil", &input);
    }

    fn _parse_soil_to_fertilizer_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("soil-to-fertilizer", &input);
    }

    fn _parse_fertilizer_to_water_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("fertilizer-to-water", &input);
    }

    fn _parse_water_to_light_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("water-to-light", &input);
    }

    fn _parse_light_to_temperature_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("light-to-temperature", &input);
    }

    fn _parse_temperature_to_humidity_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("temperature-to-humidity", &input);
    }

    fn _parse_humidity_to_location_map(input: &String) -> Vec<Vec<u64>> {
        return Day5::_parse_map_data("humidity-to-location", &input);
    }

    fn _parse_map_data(map_type: &'static str, input: &String) -> Vec<Vec<u64>> {
        let pattern = Regex::new(format!(r"{} map:\n(\d+\s*)+", map_type).as_str()).unwrap();
        let Some(captures) = pattern.captures(&input) else { return Vec::new() };
        let mut numbers: Vec<Vec<u64>> = Vec::new();

        for number_line in captures[0].split("\n") {
            let number_pattern = Regex::new(r"(\d+\s*)").unwrap();
            let numbers_on_line = number_pattern.find_iter(number_line)
            .filter_map(|digits|Some(digits.as_str().trim().to_string()))
            .map(|x|x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

            if numbers_on_line.len() > 0 {
                numbers.push(numbers_on_line);
            }

        }

        return numbers;
    }
    
}