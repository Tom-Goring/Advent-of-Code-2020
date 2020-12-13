use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_passport(raw_passport: &str) -> Option<HashMap<&str, &str>> {
    let passport = raw_passport
        .split_whitespace()
        .flat_map(|line| line.split(':'))
        .tuples()
        .collect::<HashMap<_, _>>();
    if !["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .any(|k| !passport.contains_key(k))
    {
        Some(passport)
    } else {
        None
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .filter_map(|raw_passport| parse_passport(raw_passport))
        .count() as i32
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let passports = input
        .split("\n\n")
        .filter_map(|raw_passport| parse_passport(raw_passport))
        .collect::<Vec<HashMap<&str, &str>>>();

    passports
        .iter()
        .filter(|passport| is_valid_passport(passport))
        .count() as i32
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.chars().all(|c| c.is_ascii_digit()) && v.len() == 9,
        "hgt" => {
            let height = v[..v.len() - 2].parse::<i32>().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        "cid" => true,
        _ => unreachable!(),
    })
}
