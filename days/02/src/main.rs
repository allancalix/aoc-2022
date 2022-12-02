use std::io::prelude::*;
use std::io::stdin;

#[derive(Clone, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_strategy(v: &str) -> Self {
        match v {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!("invalid expected option"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Option {
    Rock,
    Paper,
    Scissor,
}

impl Option {
    fn from_expected(v: &str) -> Self {
        match v {
            "A" => Option::Rock,
            "B" => Option::Paper,
            "C" => Option::Scissor,
            _ => unreachable!("invalid expected option"),
        }
    }

    fn from_strategy(v: &str) -> Self {
        match v {
            "X" => Option::Rock,
            "Y" => Option::Paper,
            "Z" => Option::Scissor,
            _ => unreachable!("invalid expected option"),
        }
    }

    fn selection_score(&self) -> u32 {
        match self {
            Option::Rock => 1,
            Option::Paper => 2,
            Option::Scissor => 3,
        }
    }

    fn outcome_needs(&self, result: &Outcome) -> Option {
        if result == &Outcome::Win {
            return match self {
                Option::Rock => Option::Paper,
                Option::Paper => Option::Scissor,
                Option::Scissor => Option::Rock,
            };
        }

        if result == &Outcome::Lose {
            return match self {
                Option::Rock => Option::Scissor,
                Option::Paper => Option::Rock,
                Option::Scissor => Option::Paper,
            };
        }

        return self.clone()
    }

    fn beats(&self, opponent: &Option) -> Outcome {
        if self == &Option::Rock {
            return match opponent {
                Option::Rock => Outcome::Draw,
                Option::Paper => Outcome::Lose,
                Option::Scissor => Outcome::Win,
            };
        }

        if self == &Option::Paper {
            return match opponent {
                Option::Rock => Outcome::Win,
                Option::Paper => Outcome::Draw,
                Option::Scissor => Outcome::Lose,
            };
        }

        match opponent {
            Option::Rock => Outcome::Lose,
            Option::Paper => Outcome::Win,
            Option::Scissor => Outcome::Draw,
        }
    }
}

struct Round {
    player_1: Option,
    player_2: Option,
}

impl Round {
    fn p2_score(&self) -> u32 {
        let outcome_score = match self.player_2.beats(&self.player_1) {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };

        outcome_score + self.player_2.selection_score()
    }
}

fn parse(strategy: &str, rigged: bool) -> Vec<Round> {
    let mut rounds = vec![];
    for line in strategy.lines().filter(|line| !line.is_empty()) {
        let round = {
            let mut opts = line.split(" ");
            let player_1 = Option::from_expected(opts.next().unwrap());
            let player_2 = if rigged {
                player_1.outcome_needs(&Outcome::from_strategy(opts.next().unwrap()))
            } else {
                Option::from_strategy(opts.next().unwrap())
            };

            Round {
                player_1,
                player_2,
            }
        };

        rounds.push(round);
    }

    rounds
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let game = parse(&input, true);
    let expected_score: u32 = game.iter().map(|round| round.p2_score()).sum();

    println!("Expected score - {}", expected_score);

    Ok(())
}
