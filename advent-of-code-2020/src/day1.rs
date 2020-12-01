use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    return input.lines().map(|s| s.parse().unwrap()).collect();
}

#[aoc(day1, part1)]
pub fn part1(numbers: &Vec<i32>) -> i32 {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            if num1 + num2 == 2020 {
                return num1 * num2
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(numbers: &Vec<i32>) -> i32 {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            for num3 in numbers.iter() {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3
                }
            }
        }
    }

    unreachable!()
}
