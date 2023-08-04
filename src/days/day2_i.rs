use std::str::FromStr;

use anyhow::{anyhow, Error};
use itertools::Itertools;

use super::day::Day;

pub struct Day2I {}

#[derive(Debug, Clone, Copy)]
enum End {
    Win,
    Draw,
    Loss,
}

impl End {
    fn end_points(self) -> usize {
        match self {
            End::Win => return 6,
            End::Draw => return 3,
            End::Loss => return 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn choice_points(self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }

    fn wins_against(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissor) | (Self::Paper, Self::Rock) | (Self::Scissor, Self::Paper)
        )
    }

    fn calc_points(self, opponent: Self) -> End {
        if self.wins_against(opponent) {
            return End::Win;
        } else if opponent.wins_against(self) {
            return End::Loss;
        } else {
            return End::Draw;
        }
    }
}

impl FromStr for Choice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => return Ok(Choice::Rock),
            "B" | "Y" => return Ok(Choice::Paper),
            "C" | "Z" => return Ok(Choice::Scissor),
            _ => return Err(anyhow!("Wrong choice!")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Choice,
    mine: Choice,
}

impl Round {
    fn points_for_round(self) -> usize {
        return self.mine.choice_points() + self.mine.calc_points(self.opponent).end_points();
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: Vec<String> = s
            .split(" ")
            .map(|x| x.parse::<String>().ok().unwrap())
            .collect_vec();
        return Ok(Self {
            opponent: choices.get(0).unwrap().parse::<Choice>()?,
            mine: choices.get(1).unwrap().parse::<Choice>()?,
        });
    }
}

impl Day for Day2I {
    fn run(&self) {
        // Part 1
        println!(
            "{:?}",
            std::fs::read_to_string("input/day2-rps")
                .expect("day2-rps should exist")
                .lines()
                .map(|x| x.parse::<Round>().unwrap().points_for_round())
                .sum::<usize>()
        )
    }
}
