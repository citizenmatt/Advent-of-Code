use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    return input.lines().map(|s| s.chars().map(|c| c == '#').collect()).collect()
}

fn count_trees(map: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
    return map.iter().enumerate().filter(|(i, l)| {
        i % down == 0 && l[i * right / down % l.len()]
    }).count()
}

#[aoc(day3, part1)]
pub fn part1(map: &Vec<Vec<bool>>) -> usize {
    return count_trees(map, 3, 1)
}

#[aoc(day3, part2, naive)]
pub fn part2_naive(map: &Vec<Vec<bool>>) -> usize {
    return count_trees(map, 1, 1)
        * count_trees(map, 3, 1)
        * count_trees(map, 5, 1)
        * count_trees(map, 7, 1)
        * count_trees(map, 1, 2)
}

#[aoc(day3, part2, tuples)]
pub fn part2_tuples(map: &Vec<Vec<bool>>) -> usize {
    let routes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    return routes.iter().map(|(r, d)| count_trees(map, *r, *d)).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let input = parse_input(input);

        assert_eq!(part1(&input), 7);
        assert_eq!(part2_naive(&input), 336);
        assert_eq!(part2_tuples(&input), 336);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day3.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 265);
        assert_eq!(part2_naive(&input), 3154761400);
        assert_eq!(part2_tuples(&input), 3154761400);
    }
}