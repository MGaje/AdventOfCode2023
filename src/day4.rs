#[path = "./util.rs"] mod util;

use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;
use regex::Regex;

pub fn run() {
    let run = Day4::new();
    //run.part_1();
    run.part_2();
}

pub struct Day4 {
    cards: Vec<Card>
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            cards: Vec::new()
        }
    }

    pub fn part_1(mut self) {
        self.cards = util::read_file_to_lines("./inputs/day4.txt")
            .iter()
            .map(|c| Card::deserialize(&c))
            .collect::<Vec<Card>>();

        let cards_points: i32 = self.cards.iter().cloned().map(|x|x.get_match_score()).sum();

        for card in self.cards {
            println!("Card #{} -> (Winning: {}) -> (Playing: {})", card.id, card.winning_numbers.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(","), card.playing_numbers.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(","))
        }

        println!("");
        println!("Score: {}", cards_points);
    }

    pub fn part_2(mut self) {
        self.cards = util::read_file_to_lines("./inputs/day4.txt")
            .iter()
            .map(|c| Card::deserialize(&c))
            .collect::<Vec<Card>>();

        let mut source_of_truth: HashMap<i32, i32> = HashMap::new();

        // Fill out the hashmap with initial values.
        for card in &self.cards {
            let c = card.clone();
            source_of_truth.insert(card.id, 1);
        }

        let mut map_iter = 0 as i32;
        let map_len = source_of_truth.len() as i32;
        while map_iter < map_len {

            let c = self.cards.iter().nth(map_iter as usize).unwrap();
            let card_id = c.id;

            let mut current_card_count = *source_of_truth.get(&card_id).unwrap_or(&0);
            while current_card_count > 0 {

                let current_card_match_count = c.clone().get_match_count() as i32;

                let mut i: i32 = 0;
                while i < current_card_match_count {
                    *source_of_truth.entry(card_id + i + 1).or_insert(1) += 1;

                    i += 1;
                }

                current_card_count -= 1;
            }

            map_iter += 1;
        }

        println!("source_of_truth = {:?}", source_of_truth);

        let sum: i32 = source_of_truth
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<i32>>()
            .iter()
            .sum();

        println!("Total card count: {}", sum);

    }
}

#[derive(Clone)]
pub struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    playing_numbers: Vec<i32>
}

impl Card {
    pub fn new() -> Card {
        Card{
            id: 0,
            winning_numbers: Vec::new(),
            playing_numbers: Vec::new()
        }
    }

    pub fn get_match_count(self) -> u32 {
        let playing_hs: HashSet<&i32> = HashSet::from_iter(self.playing_numbers.iter().clone());
        let winning_hs: HashSet<&i32> = HashSet::from_iter(self.winning_numbers.iter().clone());

        let matches = playing_hs.intersection(&winning_hs);
        
        return matches.count() as u32;
    }

    pub fn get_match_score(self) -> i32 {
        let match_count = self.get_match_count();

        if match_count == 0 {
            return 0;
        }

        return 2_i32.pow(match_count - 1);
    }

    pub fn deserialize(data: &String) -> Card {
        let card_re = Regex::new(r"Card\s*(\d+)\s*:(\s*[\w+|\s]+)").unwrap();
        let Some(caps) = card_re.captures(&data) else { return Card::new() };
        let id_str = &caps[1];
        let numbers_str = &caps[2];
        let mut card: Card = Card::new();

        card.id = id_str.parse::<i32>().unwrap();

        let parsed_numbers_parts = numbers_str.split("|").map(|x|x.trim().to_string()).collect::<Vec<String>>();

        card.winning_numbers = parsed_numbers_parts[0]
            .split_ascii_whitespace()
            .map(|x| x.trim().to_string())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        card.playing_numbers = parsed_numbers_parts[1]
            .split_ascii_whitespace()
            .map(|x| x.trim().to_string())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        return card;
    }
}