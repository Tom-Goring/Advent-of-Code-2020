use aoc_runner_derive::aoc;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct Rule {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl Rule {
    pub fn is_valid_p1(&self) -> bool {
        let freq = *count_chars(&self.password).get(&self.letter).unwrap_or(&0);
        freq >= self.min && freq <= self.max
    }

    pub fn is_valid_p2(&self) -> bool {
        (self.password.chars().nth(self.min as usize - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.max as usize - 1).unwrap() == self.letter)
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&line| line.parse::<Rule>().unwrap().is_valid_p1())
        .count() as i32
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&line| line.parse::<Rule>().unwrap().is_valid_p2())
        .count() as i32
}

fn count_chars(input: &str) -> HashMap<char, u32> {
    let mut freq = HashMap::new();
    for char in input.chars() {
        *freq.entry(char).or_insert(0) += 1;
    }
    freq
}
