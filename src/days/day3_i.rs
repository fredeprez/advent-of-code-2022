use std::collections::HashMap;

use super::day::Day;

pub struct Day3I {}

impl Day for Day3I {
    fn run(&self) {
        println!(
            "{:?}",
            std::fs::read_to_string("input/day3-rucksack")
                .expect("Input file should exist!")
                .lines()
                .map(|x| {
                    let (first, last) = x.split_at(x.len() / 2);

                    let mut c_first: HashMap<char, u32> = std::collections::HashMap::new();
                    for ch in first.chars() {
                        c_first
                            .entry(ch)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    }
                    let mut found: u32 = 0;
                    for ch in last.chars() {
                        if c_first.contains_key(&ch) {
                            found = ch as u32;
                        }
                    }
                    if found < 91 {
                        found = found - 38;
                    } else {
                        found = found - 96
                    }
                    return found;
                })
                .sum::<u32>()
        )
    }
}
