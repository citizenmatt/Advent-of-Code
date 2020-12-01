use aoc_runner_derive::{aoc};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let numbers = input.lines().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

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
pub fn part2(input: &str) -> i32 {
    let numbers = input.lines().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

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
