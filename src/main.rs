mod days;

use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use days::{day::Day, day1::Day1, day2_i::Day2I, day2_ii::Day2II};

enum Days {
    Day1(Day1),
    Day2I(Day2I),
    Day2II(Day2II),
}

impl FromStr for Days {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "day1" => return Ok(Days::Day1(Day1 {})),
            "day2_i" => return Ok(Days::Day2I(Day2I {})),
            "day2_ii" => return Ok(Days::Day2II(Day2II {})),
            _ => return Err(anyhow!("Wrong argument")),
        }
    }
}

impl Day for Days {
    fn run(&self) {
        match self {
            Days::Day1(day1) => day1.run(),
            Days::Day2I(day2_i) => day2_i.run(),
            Days::Day2II(day2_ii) => day2_ii.run(),
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
