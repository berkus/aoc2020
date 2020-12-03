use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
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

    assert_eq!(output.len(), 2);
    output[0] * output[1]
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    use std::collections::HashSet;
    let mut partial_sums = HashSet::new();
    let mut output = HashSet::new();
    input
        .lines()
        .map(|l| {
            let x = l.parse::<i32>().unwrap();
            for part in partial_sums.iter() {
                if partial_sums.contains(&(2020 - x - part)) {
                    output.insert(x);
                    output.insert(*part);
                    output.insert(2020 - part - x);
                }
            }
            partial_sums.insert(x); // we found x, now we need 2020 - x split between remaining numbers somewhere
        })
        .for_each(drop);

    let output = output.iter().copied().collect::<Vec<i32>>();
    assert_eq!(output.len(), 3);
    output[0] * output[1] * output[2]
}
