use super::day::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self) {
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
    }
}
