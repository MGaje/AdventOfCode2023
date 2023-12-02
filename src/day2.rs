#[path = "./util.rs"] mod util;

use regex::Regex;

pub fn run() {
    //let sum = Day2::new().part_1();
    // println!("");
    // println!("Sum of game ids is {}", sum);

    let sum = Day2::new().part_2();
    println!("");
    println!("Sum of game powers is {}", sum);
}

pub struct Day2 {
    // Empty.
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {}
    }

    pub fn part_1(self) -> i32 {
        let games = util::read_file_to_lines("./inputs/day2.txt")
            .iter()
            .map(|game_line| {
                    let g = Game::new();
                    return g.deserialize_game(&game_line);
                })
            .collect::<Vec<Game>>();

        let bag = [12, 13, 14];

        return games
            .iter()
            .map(|g| {
                    let red_count = g.get_max_red_count();
                    let green_count = g.get_max_green_count();
                    let blue_count = g.get_max_blue_count();

                    println!("Game {} -> r: {}, g: {}, b: {}", g.id, red_count, green_count, blue_count);

                    if red_count <= bag[0] && green_count <= bag[1] && blue_count <= bag[2] {
                        return g.id;
                    }

                    return 0;
                })
            .sum();
    }

    pub fn part_2(self) -> i32 {
        let games = util::read_file_to_lines("./inputs/day2.txt")
            .iter()
            .map(|game_line| {
                    let g = Game::new();
                    return g.deserialize_game(&game_line);
                })
            .collect::<Vec<Game>>();

        return games
            .iter()
            .map(|g| g.get_power())
            .sum();
    }
}

pub struct Game {
    id: i32,
    sets: Vec<GameSet>
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            sets: Vec::new()
        }
    }

    pub fn deserialize_game(mut self, game_line: &String) -> Game {
        let game_sets_str = game_line.split(":").last().unwrap().trim().to_string();

        self.id = Game::_parse_game_id(&game_line);
        self.sets = Game::_parse_game_sets(&game_sets_str);

        return self;
    }

    pub fn get_power(&self) -> i32 {
        let max_red = self.get_max_red_count();
        let max_green = self.get_max_green_count();
        let max_blue = self.get_max_blue_count();

        return max_red * max_green * max_blue;
    }

    pub fn get_max_red_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.red_count)
            .max()
            .unwrap();
    }

    pub fn get_max_green_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.green_count)
            .max()
            .unwrap();
    }

    pub fn get_max_blue_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.blue_count)
            .max()
            .unwrap();
    }

    pub fn get_min_red_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.red_count)
            .min()
            .unwrap();
    }

    pub fn get_min_green_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.green_count)
            .min()
            .unwrap();
    }

    pub fn get_min_blue_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.blue_count)
            .min()
            .unwrap();
    }

    pub fn get_total_red_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.red_count)
            .sum();
    }

    pub fn get_total_green_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.green_count)
            .sum();
    }

    pub fn get_total_blue_count(&self) -> i32 {
        return self.sets
            .iter()
            .map(|set| set.blue_count)
            .sum();
    }

    fn _parse_game_id(game_line: &String) -> i32 {
        let id_re = Regex::new(r"Game (\d+)").unwrap();
        let Some(caps) = id_re.captures(&game_line) else { return -1 };
        let id_str = &caps[1];

        return id_str.parse::<i32>().unwrap();
    }

    fn _parse_game_sets(game_sets_line: &String) -> Vec<GameSet> {
        return game_sets_line.split(";")
            .map(|line| line.trim().to_string())
            .map(|parsed_line| {
                    let red_count = Game::_parse_game_set_color_count(&parsed_line, "red");
                    let green_count = Game::_parse_game_set_color_count(&parsed_line, "green");
                    let blue_count = Game::_parse_game_set_color_count(&parsed_line, "blue");

                    return GameSet::new(red_count, green_count, blue_count);
                })
            .collect::<Vec<GameSet>>();
    }

    fn _parse_game_set_color_count(game_set_line: &String, target_color: &str) -> i32 {
        let color_count_re = Regex::new(format!(r"(\d+) {}", target_color).as_str()).unwrap();
        let Some(caps) = color_count_re.captures(&game_set_line) else { return 0 };
        let color_count_str = &caps[1];

        return color_count_str.parse::<i32>().unwrap();
    }
}

pub struct GameSet {
    red_count: i32,
    green_count: i32,
    blue_count: i32
}

impl GameSet {
    pub fn new(r: i32, g: i32, b: i32) -> GameSet {
        GameSet {
            red_count: r,
            green_count: g,
            blue_count: b
        }
    }
}