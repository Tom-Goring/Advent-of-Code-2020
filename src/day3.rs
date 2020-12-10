use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let map = generate_map(input);
    let mut current_row = 0;
    let mut current_column = 0;
    let mut trees = 0;

    while current_row != map.len() {
        if map[current_row][current_column] {
            trees += 1;
        }
        current_column += 3;
        current_column %= map.first().unwrap().len();
        current_row += 1;
    }

    trees
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let map = generate_map(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut results: Vec<u64> = Vec::new();

    for slope in slopes.iter() {
        let mut current_row = 0;
        let mut current_column = 0;
        let mut trees = 0;

        while current_row < map.len() {
            if map[current_row][current_column] {
                trees += 1;
            }
            current_column += slope.0;
            current_column %= map.first().unwrap().len();
            current_row += slope.1;
        }
        results.push(trees);
    }

    println!("{:?}", results);

    results.iter().product()
}

fn generate_map(input: &str) -> Vec<Vec<bool>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid character found on map"),
                })
                .collect()
        })
        .collect()
}
