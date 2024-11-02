//! 2023 Day 1

elvish::day!(1);

#[elvish::part(1)]
fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));

            let a = iter.next().unwrap();
            let b = iter.last().unwrap_or(a);

            (a * 10 + b) as i32
        })
        .sum()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_slice(slice: &str) -> Option<i32> {
    if let Some(result) = slice.chars().next().unwrap().to_digit(10) {
        return Some(result as i32);
    }

    for (i, digit) in DIGITS.iter().enumerate() {
        if slice.starts_with(digit) {
            return Some(i as i32 + 1);
        }
    }

    None
}

#[elvish::part(2)]
fn part2(input: &str) -> i32 {
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

// TODO: Figure out example layout
// elvish::examples! {
//     part1: "
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet
//     " => 142,
//
//     part2: "
//         two1nine
//         eightwothree
//         abcone2threexyz
//         xtwone3four
//         4nineeightseven2
//         zoneight234
//         7pqrstsixteen
//     " => 281,
// }
