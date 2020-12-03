use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut memory = vec![];
    let mut output = vec![];
    input
        .lines()
        .map(|l| {
            let x = l.parse::<i32>().unwrap();
            if memory.contains(&x) {
                output.push(x);
                output.push(2020 - x);
            }
            let target = 2020 - x;
            memory.push(target); // we found x, now we need 2020 - x somewhere
        })
        .for_each(drop);
    output
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    assert_eq!(input.len(), 2);
    input[0] * input[1]
}
