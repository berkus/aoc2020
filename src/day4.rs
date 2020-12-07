use {aoc_runner_derive::aoc, maplit::hashmap};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let fields = hashmap! {
        "byr" => 0b0000_0001, // (Birth Year)
        "iyr" => 0b0000_0010, // (Issue Year)
        "eyr" => 0b0000_0100, // (Expiration Year)
        "hgt" => 0b0000_1000, // (Height)
        "hcl" => 0b0001_0000, // (Hair Color)
        "ecl" => 0b0010_0000, // (Eye Color)
        "pid" => 0b0100_0000, // (Passport ID)
        "cid" => 0b1000_0000, // (Country ID)
    };
    let mut count = 0;
    let mut flags: u8 = 0;
    input
        .lines()
        .map(|l| {
            if l.is_empty() {
                // Check map for all necessary fields
                if flags & 0b0111_1111 == 0b0111_1111 {
                    // Correct
                    count += 1;
                }
                flags = 0;
            } else {
                // Grab all fields, stuff em into map
                for field in l.split(' ') {
                    let f = field.split(':').next().unwrap();
                    flags |= fields.get(f).unwrap();
                }
            }
        })
        .for_each(drop);
    // Last input
    if flags & 0b0111_1111 == 0b0111_1111 {
        // Correct
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sample() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        assert_eq!(part1(input), 2);
    }
}
