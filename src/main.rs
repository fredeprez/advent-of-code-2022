mod days;

use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use days::{day::Day, day1::Day1, day2::Day2};

enum Days {
    Day1(Day1),
    Day2(Day2),
}

impl FromStr for Days {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "day1" => return Ok(Days::Day1(Day1 {})),
            "day2" => return Ok(Days::Day2(Day2 {})),
            _ => return Err(anyhow!("Wrong argument")),
        }
    }
}

impl Day for Days {
    fn run(&self) {
        match self {
            Days::Day1(day1) => day1.run(),
            Days::Day2(day2) => day2.run(),
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
