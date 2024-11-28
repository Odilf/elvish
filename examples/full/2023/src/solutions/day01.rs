// Generates:
//
// ```rust
// impl elvish::solution::Part<1, 1> for crate::Solutions {
//     fn solve(input: &str) -> impl std::fmt::Display {
//         part1(input)
//     }
// }
//
// #[test]
// fn part1_example() {
//     assert_eq!(part1(EXAMPLE_PART1), 142)
// }
// ```
#[elvish::solution(day = 1, example = 142)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));

            let a = iter.next().unwrap();
            let b = iter.last().unwrap_or(a);

            a * 10 + b
        })
        .sum()
}

// Generates:
//
// ```rust
// impl elvish::solution::Part<2, 1> for crate::Solutions {
//     fn solve(input: &str) -> impl std::fmt::Display {
//         part2(input)
//     }
// }
//
// #[test]
// fn part2_example() {
//     assert_eq!(part2(EXAMPLE_PART2), 281)
// }
// ```
#[elvish::solution(day = 1, example = 281)]
fn part2(input: &str) -> u32 {
    let parse_slice = |slice: &str| {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let digit = slice.chars().next().unwrap().to_digit(10);
        let named = || {
            digits
                .iter()
                .enumerate()
                .find(|(_, &digit)| slice.starts_with(digit))
                .map(|(i, _)| i as u32 + 1)
        };

        digit.or_else(named)
    };

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = (0..line.len()).filter_map(|i| parse_slice(&line[i..]));

            let a = iter.next().unwrap();
            let b = iter.last().unwrap_or(a);

            a * 10 + b
        })
        .sum()
}

// Generates:
//
// ```rust
// const PART1_EXAMPLE1: &str = indoc!(/* example1 */);
// const PART1_EXAMPLE: &str = PART1_EXAMPLE1;
//
// const PART2_EXAMPLE1: &str = indoc!(/* example2 */);
// const PART2_EXAMPLE: &str = PART2_EXAMPLE1;
// ```
elvish::example!(
    part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ", 

    part2: "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "
);
