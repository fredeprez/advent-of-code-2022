use std::collections::HashSet;

use itertools::Itertools;

use super::day::Day;

pub struct Day3II {}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Item(u8);

// bytes solution thanks to https://fasterthanli.me/series/advent-of-code-2022/part-3
impl TryFrom<u8> for Item {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(anyhow::anyhow!("Wrong choice!")),
        }
    }
}

impl Item {
    fn priority(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

impl Day for Day3II {
    fn run(&self) {
        let prio: usize = std::fs::read_to_string("input/day3-rucksack")
            .expect("File should exist")
            .lines()
            .map(|line| {
                return line
                    .bytes()
                    .map(|x| x.try_into().unwrap())
                    .collect::<HashSet<Item>>();
            })
            .chunks(3)
            .into_iter()
            .map(|chunks| {
                return chunks
                    // Need to use cloned(), otherwise .collect returns type HashSet<&Item>
                    .reduce(|mut set_a, set_b| set_a.intersection(&set_b).cloned().collect())
                    .expect("We always have per 3 chunks")
                    .iter()
                    .next()
                    .expect("Always one common")
                    .priority();
            })
            .sum();

        println!("{prio:?}")
    }
}
