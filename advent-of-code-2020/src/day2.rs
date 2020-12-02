use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Password {
    policy: Policy,
    password: String
}

#[derive(Debug, Default)]
pub struct Policy {
    num1: usize,
    num2: usize,
    letter: char
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Password> {
    let re = Regex::new("(?P<num1>\\d+)-(?P<num2>\\d+)\\s(?P<letter>.):\\s(?P<password>.*)").unwrap();
    return input.lines().map(|s| {
        let caps = re.captures(s).unwrap();
        Password {
            policy: Policy {
                num1: caps["num1"].parse().unwrap(),
                num2: caps["num2"].parse().unwrap(),
                letter: caps["letter"].chars().exactly_one().unwrap()
            },
            password: caps["password"].to_string()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &Vec<Password>) -> i32 {
    let mut valid_count = 0;
    for p in passwords.iter() {
        let count = p.password.chars().filter(|c| c == &p.policy.letter).count();
        if count >= p.policy.num1 && count <= p.policy.num2 {
            valid_count += 1;
        }
    }

    return valid_count
}

#[aoc(day2, part2)]
pub fn part2(passwords: &Vec<Password>) -> i32 {
    let mut valid_count = 0;

    for p in passwords.iter() {
        let password_bytes = p.password.as_bytes();
        let p1 = password_bytes[p.policy.num1 - 1] as char;
        let p2 = password_bytes[p.policy.num2 - 1] as char;
        if (p1 == p.policy.letter && p2 != p.policy.letter)
            || (p1 != p.policy.letter && p2 == p.policy.letter) {

            valid_count += 1
        }
    }

    return valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let input = parse_input(input);

        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day2.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 460);
        assert_eq!(part2(&input), 251);
    }
}
