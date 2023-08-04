use itertools::Itertools;

use super::day::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self) {
        // Part 1
        let mut max: u32 = 0;
        let mut current_max: u32 = 0;
        std::fs::read_to_string("input/elves")
            .expect("Elves file should exist")
            .lines()
            .for_each(|line| {
                let curr_line: u32 = line.parse::<u32>().unwrap_or(0);
                if curr_line > 0 {
                    current_max += curr_line;
                } else {
                    if current_max > max {
                        max = current_max;
                    }
                    current_max = 0;
                }
            });

        println!("{}", max);

        // Part 2
        println!(
            "{:?}",
            include_str!("../../input/elves")
                .split("\n\n")
                .map(|x| x
                    .lines()
                    .map(|cal| cal.parse::<u32>().unwrap())
                    .sum::<u32>())
                .map(std::cmp::Reverse)
                .k_smallest(3)
                .map(|x| x.0)
                .sum::<u32>()
        )
    }
}
