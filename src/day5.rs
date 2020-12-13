use aoc_runner_derive::aoc;
use itertools::Itertools;

fn parse_seat(seat: &str) -> i32 {
    seat.chars().fold(0, |acc, c| match c {
        'F' | 'L' => (acc << 1),
        'B' | 'R' => (acc << 1) | 1,
        _ => unreachable!(),
    })
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .map(|seat| parse_seat(seat))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    input
        .split('\n')
        .map(|seat| parse_seat(seat))
        .sorted()
        .tuple_windows()
        .take_while(|(seat1, seat2)| seat1 + 2 != *seat2)
        .last()
        .unwrap()
        .1
        + 1
}
