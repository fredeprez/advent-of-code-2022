use std::str::FromStr;

use anyhow::{anyhow, Error};
use itertools::Itertools;

use super::day::Day;

pub struct Day2II {}

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

impl FromStr for End {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => return Ok(End::Loss),
            "Y" => return Ok(End::Draw),
            "Z" => return Ok(End::Win),
            _ => return Err(anyhow!("Wrong end!")),
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
    const ALL_CHOICES: [Choice; 3] = [Choice::Rock, Choice::Paper, Choice::Scissor];

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

    fn find_winning_choice(self) -> Self {
        return Self::ALL_CHOICES
            .iter()
            .copied()
            .find(|c| c.wins_against(self))
            .expect("Should have a winning option");
    }

    fn find_losing_choice(self) -> Self {
        return Self::ALL_CHOICES
            .iter()
            .copied()
            .find(|&c| self.wins_against(c))
            .expect("Should have a losing option");
    }

    fn find_drawing_choice(self) -> Self {
        return self;
    }
}

impl FromStr for Choice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => return Ok(Choice::Rock),
            "B" => return Ok(Choice::Paper),
            "C" => return Ok(Choice::Scissor),
            _ => return Err(anyhow!("Wrong choice!")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Choice,
    end: End,
}

impl Round {
    fn points_for_choice_with_end(self) -> usize {
        let mine = match self.end {
            End::Win => self.opponent.find_winning_choice(),
            End::Loss => self.opponent.find_losing_choice(),
            End::Draw => self.opponent.find_drawing_choice(),
        };
        return mine.choice_points() + self.end.end_points();
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
            end: choices.get(1).unwrap().parse::<End>()?,
        });
    }
}

impl Day for Day2II {
    fn run(&self) {
        // Part 1
        println!(
            "{:?}",
            std::fs::read_to_string("input/day2-rps")
                .expect("day2-rps should exist")
                .lines()
                .map(|x| x.parse::<Round>().unwrap().points_for_choice_with_end())
                .sum::<usize>()
        )
    }
}
