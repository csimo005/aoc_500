use aoc::year2015::day04::*;

const EXAMPLE: &str = "abcdef";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 609043);
}
