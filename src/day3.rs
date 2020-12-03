use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
pub fn input_generator_1(input: &str) -> i32 {
    let mut total_trees = 0;
    let mut pos = 0;
    input
        .lines()
        .map(|l| {
            let modulo = l.len();
            if l.chars().nth(pos) == Some('#') {
                total_trees += 1;
            }
            pos = (pos + 3) % modulo;
        })
        .for_each(drop);
    total_trees
}

#[aoc(day3, part1)]
pub fn part1(input: &i32) -> i32 {
    *input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(input_generator_1("...#.\n..#..\n.##..\n"), 1);
        assert_eq!(input_generator_1("...#.#\n..#...\n###...\n"), 1);
    }
}
