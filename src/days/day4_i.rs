use std::collections::HashSet;

use itertools::Itertools;

use super::day::Day;

pub struct Day4I {}

#[derive(Debug)]
struct Section(usize, usize);

impl TryFrom<&str> for Section {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // todo: fix match statement: Err is unreachable.
        let tup: (usize, usize) = value
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .expect("Always has a split on -");
        match tup {
            (start, end) => Ok(Section(start, end)),
            _ => Err(anyhow::anyhow!("Can't parse numbers to section")),
        }
    }
}

impl Day for Day4I {
    fn run(&self) {
        let sum: usize = std::fs::read_to_string("input/day4-cleanup-pairs")
            .expect("day4 file to exist")
            .lines()
            .map(|x| {
                let t: (&str, &str) = x.split(",").collect_tuple().expect("Works every time");
                let first_elf = Section::try_from(t.0).unwrap();
                let last_elf = Section::try_from(t.1).unwrap();

                return (
                    HashSet::from_iter(first_elf.0..=first_elf.1),
                    HashSet::from_iter(last_elf.0..=last_elf.1),
                );
            })
            .map(|tup: (HashSet<usize>, HashSet<usize>)| {
                // part 1
                // return (tup.0.is_subset(&tup.1) || tup.1.is_subset(&tup.0)) as usize;
                // part 2
                return (!tup.0.is_disjoint(&tup.1)) as usize;
            })
            .sum();

        println!("sum: {sum:?}")
    }
}
