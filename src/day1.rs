use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let entries = hash_input(input);
    for entry in &entries {
        let target = 2020 - entry;
        if entries.contains(&target) {
            return (target * entry) as i32;
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let entries = hash_input(input);
    for entry in &entries {
        for second_entry in &entries {
            let target = 2020 - (entry + second_entry);
            if entries.contains(&target) {
                return (target * entry * second_entry) as i32;
            }
        }
    }

    unreachable!()
}

fn hash_input(input: &str) -> HashSet<i32> {
    let mut entries: HashSet<i32> = HashSet::new();
    input
        .split('\n')
        .map(|str| str.parse())
        .filter_map(Result::ok)
        .for_each(|entry| {
            entries.insert(entry);
        });
    entries
}
