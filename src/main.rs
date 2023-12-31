mod days;

use anyhow::{anyhow, Error, Result};
use days::{
    day::Day, day1::Day1, day2_i::Day2I, day2_ii::Day2II, day3_i::Day3I, day3_ii::Day3II,
    day4_i::Day4I,
};
use std::str::FromStr;

enum Days {
    Day1(Day1),
    Day2I(Day2I),
    Day2II(Day2II),
    Day3I(Day3I),
    Day3II(Day3II),
    Day4I(Day4I),
}

impl FromStr for Days {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "day1" => return Ok(Days::Day1(Day1 {})),
            "day2_i" => return Ok(Days::Day2I(Day2I {})),
            "day2_ii" => return Ok(Days::Day2II(Day2II {})),
            "day3_i" => return Ok(Days::Day3I(Day3I {})),
            "day3_ii" => return Ok(Days::Day3II(Day3II {})),
            "day4_i" => return Ok(Days::Day4I(Day4I {})),
            _ => return Err(anyhow!("Wrong day argument")),
        }
    }
}

impl Day for Days {
    fn run(&self) {
        match self {
            Days::Day1(day1) => day1.run(),
            Days::Day2I(day2_i) => day2_i.run(),
            Days::Day2II(day2_ii) => day2_ii.run(),
            Days::Day3I(day3_i) => day3_i.run(),
            Days::Day3II(day3_ii) => day3_ii.run(),
            Days::Day4I(day4_i) => day4_i.run(),
        }
    }
}

fn main() -> Result<()> {
    // parse + run
    // let day: String = std::env::args().nth(1).unwrap_or("empty".to_string());
    let day = std::env::args()
        .nth(1)
        .unwrap_or(String::from(""))
        .parse::<Days>();

    day?.run();

    return Ok(());
}
