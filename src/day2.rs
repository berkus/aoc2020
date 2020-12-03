use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
pub fn input_generator_1(input: &str) -> i32 {
    let mut total_good = 0;
    input
        .lines()
        .map(|l| {
            let mut s = l.split(':');
            let (policy, password) = (s.next().unwrap(), s.next().unwrap());
            let mut s = policy.split(' ');
            let (counts, letter) = (s.next().unwrap(), s.next().unwrap());
            let mut s = counts.split('-').map(str::parse::<i32>);
            let (min, max) = (s.next().unwrap().unwrap(), s.next().unwrap().unwrap());
            let mut count = 0;
            for cc in password.chars() {
                if letter.starts_with(cc) {
                    count += 1
                }
            }
            if count >= min && count <= max {
                total_good += 1;
            }
        })
        .for_each(drop);
    total_good
}

#[aoc(day2, part1)]
pub fn part1(input: &i32) -> i32 {
    *input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(input_generator_1("1-2 a: aaa"), 0);
        assert_eq!(input_generator_1("1-2 a: aa"), 1);
        assert_eq!(input_generator_1("2-3 a: a"), 0);
    }
}
