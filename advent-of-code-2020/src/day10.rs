use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<i32> {
    return input.lines().map(|s| s.parse().unwrap()).sorted().collect()
}

#[aoc(day10, part1)]
pub fn part1(numbers: &Vec<i32>) -> i32 {
    let mut count1 = 0;
    let mut count3 = 1;

    let mut previous = 0;

    for current in numbers {
        match current - previous {
            1 => count1 = count1 + 1,
            3 => count3 = count3 + 1,
            _ => unreachable!()
        }
        previous = current.to_owned()
    }

    return count1 * count3;
}

// Walk the list backwards
// Total = 8
// 22
// 19. Can only be 19 -> 22, so count for 19 is 1
// 16. Can only be 16 -> 19, so count for 16 is 1
// 15. Can only be 15 -> 16, count = 1
// 12. Can only be 12 -> 15, count = 1
// 11. Can only be 11 -> 12, count = 1
// 10. Can be 10 -> 11 and 10 -> 12, count = 2
// 7. Can only be 7 -> 10, count = 1
// 6. Can only be 6 -> 7, count = 1
// 5. Can be 5 -> 6, 5 -> 7 count = 2
// 4. Can be 4 -> 5, 4 -> 6, 4 -> 7 count = 3
// 1. Can only be 1 -> 4, count = 1
// 0. Can only be 0 -> 1, count = 1

// 1 route from 12 -> end
// 2 routes from 10 -> end (via 11, via 12)
// 2 routes from 7 -> end (2 via 10)
// 2 routes from 6 -> end (2 via 7)
// 2*2 = 4 routes from 5 -> end (2 via 6, 2 via 7)
// 8 from 4 -> end (4 via 5, 2 via 6, 2 via 7)
// 1 route from beginning to 4

// Walk backwards
// 22 c=1
// 19 c=1
// 16 c=1
// 15 c=1
// 12 c=1
// 11 c=1
// 10 c=2
// 7 c=2
// 6 c=2
// 5 c=2+2
// 4 c=4+2+2
// 1 c=8
// 0 c=8

#[aoc(day10, part2)]
fn part2(numbers: &Vec<i32>) -> i64 {
    // Walk the list backwards and count the possible routes from current to the last node. The
    // results are cached as they are calculated for easy lookup. The route count is the sum of the
    // route counts of the reachable nodes.
    let mut counts: HashMap<i32, i64> = HashMap::new();

    counts.insert(numbers.last().unwrap() + 3, 1);
    for number in numbers.iter().rev() {
        let mut count = 0;

        counts.get(&(number + 1)).map(|i| count = count + i);
        counts.get(&(number + 2)).map(|i| count = count + i);
        counts.get(&(number + 3)).map(|i| count = count + i);
        counts.insert(number.to_owned(), count);
    }

    let mut count = 0;
    counts.get(&1).map(|i| count = count + i);
    counts.get(&2).map(|i| count = count + i);
    counts.get(&3).map(|i| count = count + i);
    return count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        let input = parse_input(input);

        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn example2() {
        let input = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        let input = parse_input(input);

        assert_eq!(part1(&input), 220);
        assert_eq!(part2(&input), 19208);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day10.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 2470);
        assert_eq!(part2(&input), 1973822685184);
    }
}
