use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    return input.lines().map(|s| s.parse().unwrap()).sorted().collect()
}

#[aoc(day1, part1, naive)]
pub fn part1_naive(numbers: &Vec<i32>) -> i32 {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            if num1 + num2 == 2020 {
                return num1 * num2
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part1, two_pointer_algorithm)]
pub fn part1_two_pointer_algorithm(numbers: &Vec<i32>) -> i32 {

    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        if numbers[i] + numbers[j] == 2020 {
            return numbers[i] * numbers[j]
        }

        if numbers[i] + numbers[j] < 2020 {
            // Sum is less, use higher values by incrementing i
            i = i + 1
        }
        else {
            // Sum is greater than, use smaller values by decrementing j
            j = j - 1
        }
    }

    unreachable!()
}

#[aoc(day1, part2, naive)]
pub fn part2_naive(numbers: &Vec<i32>) -> i32 {
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

#[aoc(day1, part2, three_pointer_algorithm)]
pub fn part2_three_pointer_algorithm(numbers: &Vec<i32>) -> i32 {

    for num1 in numbers.iter() {

        // Now use the two pointer algorithm to find the values
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while i < j {
            if num1 + numbers[i] + numbers[j] == 2020 {
                return num1 * numbers[i] * numbers[j]
            }

            if num1 + numbers[i] + numbers[j] < 2020 {
                i = i + 1
            }
            else {
                j = j - 1
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1721
979
366
299
675
1456"#;
        let input = parse_input(input);

        assert_eq!(part1_naive(&input), 514579);
        assert_eq!(part1_two_pointer_algorithm(&input), 514579);
        assert_eq!(part2_naive(&input), 241861950);
        assert_eq!(part2_three_pointer_algorithm(&input), 241861950);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day1.txt");
        let input = parse_input(input);

        assert_eq!(part1_naive(&input), 786811);
        assert_eq!(part1_two_pointer_algorithm(&input), 786811);
        assert_eq!(part2_naive(&input), 199068980);
        assert_eq!(part2_three_pointer_algorithm(&input), 199068980);
    }
}