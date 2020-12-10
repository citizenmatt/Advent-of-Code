use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i64> {
    return input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(numbers: &Vec<i64>) -> i64 {
    return do_part1(25, numbers)
}

fn do_part1(preamble: usize, numbers: &Vec<i64>) -> i64 {

    for i in preamble..numbers.len() {
        let mut pp = vec![0; preamble];
        pp.copy_from_slice(&numbers[i - preamble..i]);
        pp.sort();
        if !two_pointer_algorithm(pp.as_slice(), numbers[i]) {
            return numbers[i]
        }
    }

    unreachable!()
}

fn two_pointer_algorithm(numbers: &[i64], target: i64) -> bool {

    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        if numbers[i] + numbers[j] == target {
            return true
        }

        if numbers[i] + numbers[j] < target {
            i = i + 1
        }
        else {
            j = j - 1
        }
    }

    return false
}

#[aoc(day9, part2, initial)]
pub fn part2(numbers: &Vec<i64>) -> i64 {
    do_part2(25, numbers)
}

fn do_part2(preamble: usize, numbers: &Vec<i64>) -> i64 {

    let target = do_part1(preamble, numbers);

    for start in 0..numbers.len() {
        if let Some(end) = find_something(numbers, start, target) {
            let run = &numbers[start..end];
            let min = run.iter().min().unwrap();
            let max = run.iter().max().unwrap();
            return min + max
        }
    }

    unreachable!()
}

fn find_something(numbers: &Vec<i64>, start: usize, target: i64) -> Option<usize> {

    let mut acc = 0;
    for (i, num) in numbers.iter().skip(start).enumerate() {
        acc = acc + num;
        if acc == target {
            return Some(i + start)
        }
    }

    return None
}

#[aoc(day9, part2, better)]
pub fn part2_better(numbers: &Vec<i64>) -> i64 {
    do_part2_better(25, numbers)
}

fn do_part2_better(preamble: usize, numbers: &Vec<i64>) -> i64 {
    let target = do_part1(preamble, numbers);

    let mut start = 0;
    let mut end = 0;
    let mut acc = numbers[start];

    while acc != target && end < numbers.len() && start < numbers.len() {
        if acc < target {
            end = end + 1;
            acc = acc + numbers[end];
        }
        else if acc > target {
            acc = acc - numbers[start];
            start = start + 1;
        }
    }

    if acc == target {
        let run = &numbers[start..end];
        if let MinMax(min, max) = run.iter().minmax() {
            return min + max
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        let input = parse_input(input);

        assert_eq!(do_part1(5, &input), 127);
        assert_eq!(do_part2(5, &input), 62);
        assert_eq!(do_part2_better(5, &input), 62);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day9.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 1721308972);
        assert_eq!(part2(&input), 209694133);
        assert_eq!(part2_better(&input), 209694133);
    }
}
