// type | opponent  | response | scores
//  A   | Rock      |    X     | 1
//  B   | Paper     |    Y     | 2
//  C   | Scissors  |    Z     | 3
// _________________________________
// lost                        | 0
// draw                        | 3
// won                         | 6

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl Figure {
    fn value(&self) -> usize {
        match *self {
            Figure::Rock => 1,
            Figure::Paper => 2,
            Figure::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct SingleRound {
    opponent: Figure,
    response: Figure,
    score: usize,
    strategy: Strategy,
    strategy_score: usize,
}

impl SingleRound {
    fn translate(c: &str) -> Figure {
        match c {
            "A" => Figure::Rock,
            "B" => Figure::Paper,
            "C" => Figure::Scissors,
            "X" => Figure::Rock,
            "Y" => Figure::Paper,
            "Z" => Figure::Scissors,
            _ => panic!("invalid character: {}", c),
        }
    }
    fn get_total_score(opponent: &Figure, response: &Figure) -> usize {
        match opponent {
            Figure::Rock => match response {
                Figure::Rock => 3 + response.value(),
                Figure::Paper => 6 + response.value(),
                Figure::Scissors => response.value(),
            },
            Figure::Paper => match response {
                Figure::Rock => response.value(),
                Figure::Paper => 3 + response.value(),
                Figure::Scissors => 6 + response.value(),
            },
            Figure::Scissors => match response {
                Figure::Rock => 6 + response.value(),
                Figure::Paper => response.value(),
                Figure::Scissors => 3 + response.value(),
            },
        }
    }
    fn translate_strategy(response: &str) -> Strategy {
        match response {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("invalid character: {}", response),
        }
    }
    fn get_strategy_score(opponent: &Figure, response: &Strategy) -> usize {
        match opponent {
            Figure::Rock => match response {
                Strategy::Lose => Figure::Scissors.value(),
                Strategy::Draw => Figure::Rock.value() + 3,
                Strategy::Win => Figure::Paper.value() + 6,
            },
            Figure::Paper => match response {
                Strategy::Lose => Figure::Rock.value(),
                Strategy::Draw => Figure::Paper.value() + 3,
                Strategy::Win => Figure::Scissors.value() + 6,
            },
            Figure::Scissors => match response {
                Strategy::Lose => Figure::Paper.value(),
                Strategy::Draw => Figure::Scissors.value() + 3,
                Strategy::Win => Figure::Rock.value() + 6,
            },
        }
    }
    fn new(opponent: &str, response: &str) -> Self {
        let opponent = Self::translate(opponent);
        let strategy = Self::translate_strategy(&response);
        let response = Self::translate(response);
        let score = Self::get_total_score(&opponent, &response);
        let strategy_score = Self::get_strategy_score(&opponent, &strategy);
        SingleRound {
            opponent,
            response,
            score,
            strategy,
            strategy_score,
        }
    }
}

impl FromStr for SingleRound {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (o, r) = s.split_once(' ').unwrap();
        println!("{:?}", (o, r));
        Ok(Self::new(o, r))
    }
}

fn main() {
    let scores: Vec<SingleRound> = include_str!("./input_day2.txt")
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            return Some(SingleRound::from_str(line).ok());
        })
        .flatten()
        .collect();

    let total: usize = scores.iter().map(|s| s.score).sum();

    println!("{:?}", scores);
    println!("first part: {:?}", total);

    let total: usize = scores.iter().map(|s| s.strategy_score).sum();

    println!("second part: {:?}", total);
}
