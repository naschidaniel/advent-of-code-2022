use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day03.txt");

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn solution_day03() {
    let input = read_input_file();
    let rucksack = Rucksacks::init(input);
    println!(
        "The solution for the 1st part of the puzzle from day 03 is '{:?}'!",
        rucksack.sum_priorities()
    );
}

struct Items {
    first: String,
    second: String,
}

impl Items {
    fn new(line: String) -> Self {
        let (first, second) = line.split_at(line.len() / 2);
        Self {
            first: first.to_string(),
            second: second.to_string(),
        }
    }

    fn get_unique_item(&self) -> char {
        let unique_first = self.first.chars().collect::<HashSet<char>>();
        let unique_second = self.second.chars().collect::<HashSet<char>>();
        unique_first
            .intersection(&unique_second).copied()
            .collect::<Vec<_>>()[0]
    }
}

struct Rucksacks {
    item_list: Vec<char>,
    priority_map: HashMap<char, i32>,
}

impl Rucksacks {
    fn init(input: String) -> Self {
        let mut priority_map = HashMap::new();
        for (index, character) in ('a'..='z').into_iter().enumerate() {
            priority_map.insert(character, index as i32 + 1);
        }

        for (index, character) in ('A'..='Z').into_iter().enumerate() {
            priority_map.insert(character, index as i32 + 26 + 1);
        }

        let mut item_list = Vec::new();
        for line in input.lines() {
            let rucksack = Items::new(line.to_string());
            item_list.push(rucksack.get_unique_item())
        }
        Self {
            item_list,
            priority_map,
        }
    }

    fn priorities(&self) -> Vec<i32> {
        self.item_list
            .iter()
            .map(|x| self.priority_map[x])
            .collect()
    }

    fn sum_priorities(&self) -> i32 {
        self.priorities().iter().sum()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        let rucksack = Rucksacks::init(input);
        assert_eq!(
            vec!['p', 'L', 'P', 'v', 't', 's'],
            rucksack.item_list
        );
        assert_eq!(vec![16, 38, 42, 22, 20, 19], rucksack.priorities());
        assert_eq!(157, rucksack.sum_priorities());
    }
}
