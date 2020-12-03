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

#[aoc_generator(day3, part2)]
pub fn input_generator_2(input: &str) -> [i64; 5] {
    let mut sums = [0i64; 5]; // right1_down1, right3_down1, right5_down1, right7_down1, right1_down2
    let mut poss = [0usize; 5];
    let mut flip = true;
    let incr = [1usize, 3, 5, 7, 1];
    input
        .lines()
        .map(|l| {
            let modulo = l.len();
            for idx in 0..=4 {
                if idx == 4 {
                    if !flip {
                        flip = true;
                        continue;
                    } else {
                        flip = false;
                    }
                }
                if l.chars().nth(poss[idx]) == Some('#') {
                    sums[idx] += 1;
                }
                poss[idx] = (poss[idx] + incr[idx]) % modulo;
            }
        })
        .for_each(drop);
    println!("{:?}", sums);
    sums
}

#[aoc(day3, part1)]
pub fn part1(input: &i32) -> i32 {
    *input
}

#[aoc(day3, part2)]
pub fn part2(input: &[i64]) -> i64 {
    input[0] * input[1] * input[2] * input[3] * input[4]
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
