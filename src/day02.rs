use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day02.txt");

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn solution_day02() {
    let input = read_input_file();
    let mut game = Game::init(input);
    game.play();
    println!(
        "The solution for the 1st part of the puzzle from day 02 is '{:?}'!",
        game.get_total("player_2")
    );
}
struct Game {
    total_player_1: i32,
    total_player_2: i32,
    strategy: Vec<(String, String)>,
}

impl Game {
    /// Rules
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
    // // (Me | Response)
    // // (A | X) Rock ... 1
    // // (B | Y) Paper ... 2
    // // (C | Z) Scissors ... 3

    // // Win ... 6
    // // Draw ... 3
    // // Loose ... 0
    fn init(input: String) -> Self {
        let total_player_1 = 0;
        let total_player_2 = 0;
        let strategy = input
            .lines()
            .map(|l| {
                let mut iter = l.splitn(2, ' ');
                let input_1 = match iter.next().unwrap() {
                    "A" => "Rock",
                    "B" => "Paper",
                    "C" => "Scissors",
                    _ => panic!("not implemented"),
                }
                .to_string();
                let input_2 = match iter.next().unwrap() {
                    "X" => "Rock",
                    "Y" => "Paper",
                    "Z" => "Scissors",
                    _ => panic!("not implemented"),
                }
                .to_string();
                (input_1, input_2)
            })
            .collect();
        Game {
            total_player_1,
            total_player_2,
            strategy,
        }
    }

    fn play(&mut self) {
        for comparison in self.strategy.iter() {
            let input_1 = comparison.to_owned().0;
            let input_2 = comparison.to_owned().1;
            let winner = if input_1 == input_2 {
                "draw"
            } else if (input_1 == "Rock" && input_2 == "Scissors")
                || (input_1 == "Scissors" && input_2 == "Paper")
                || (input_1 == "Paper" && input_2 == "Rock")
            {
                "player_1"
            } else if (input_1 == "Paper" && input_2 == "Scissors")
                || (input_1 == "Scissors" && input_2 == "Rock")
                || (input_1 == "Rock" && input_2 == "Paper")
            {
                "player_2"
            } else {
                panic!("The rule for {} against {} is missing!", input_1, input_2)
            };

            self.total_player_1 = match winner {
                "player_1" => self.total_player_1 + 6 + Game::get_points(input_1),
                "draw" => self.total_player_1 + 3 + Game::get_points(input_1),
                "player_2" => self.total_player_1 + Game::get_points(input_1),
                _ => panic!("The winner is not implemented!"),
            };

            self.total_player_2 = match winner {
                "player_2" => self.total_player_2 + 6 + Game::get_points(input_2),
                "draw" => self.total_player_2 + 3 + Game::get_points(input_2),
                "player_1" => self.total_player_2 + Game::get_points(input_2),
                _ => panic!("The winner is not implemented!"),
            };
        }
    }

    fn get_points(input: String) -> i32 {
        match input.as_str() {
            "Rock" => 1,
            "Paper" => 2,
            "Scissors" => 3,
            _ => panic!("Not implemented"),
        }
    }

    fn get_total(&self, player: &str) -> i32 {
        match player {
            "player_1" => self.total_player_1,
            "player_2" => self.total_player_2,
            _ => panic!("The total fot the {} is not implemented", player),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_rules() {
        let input = "A Y
B X
C Z"
        .to_string();
        let mut game = Game::init(input);
        game.play();
        assert_eq!(15, game.get_total("player_1"));
    }
}
