use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day01.txt");

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn solution_day01() {
    let input = read_input_file();
    let elves = Elves::init(input);
    println!(
        "The solution for the 1st part of the puzzle from day 01 is '{:?}'!",
        elves.get_max()
    );
}

#[derive(Debug, Clone)]
struct Elv {
    calories: Vec<i32>,
}

impl Elv {
    fn init(input: String) -> Self {
        let calories = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
        Elv { calories }
    }

    fn get_total(&self) -> i32 {
        self.calories.iter().sum::<i32>()
    }
}

struct Elves {
    members: Vec<Elv>,
}

impl Elves {
    fn init(input: String) -> Self {
        let members = input
            .split("\n\n")
            .map(|e| Elv::init(e.to_string()))
            .collect();
        Elves { members }
    }

    fn list_totals(&self) -> Vec<i32> {
        self.members.iter().map(|m| m.get_total()).collect()
    }

    fn get_max(&self) -> i32 {
        *self.list_totals().iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_init_elves() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();
        let elves = Elves::init(input);
        let result = elves.list_totals();
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(expected, result);
        let maximum = elves.get_max();
        assert_eq!(24000, maximum);
    }
}
