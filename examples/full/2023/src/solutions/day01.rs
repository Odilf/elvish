// Generates:
//
// ```rust
// impl elvish::solution::Part<1, 1> for crate::Solutions {
//     fn solve(input: &str) -> impl std::fmt::Display {
//         part1(input)
//     }
// }
//
// impl elvish::solution::Part<2, 1> for crate::Solutions {
//     fn solve(input: &str) -> impl std::fmt::Display {
//         part2(input)
//     }
// }
// ```

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

fn parse_slice(slice: &str) -> Option<i32> {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

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

// #[cfg(test)]
// mod test {
//     #[test]
//     fn example1_1() {
//         EXAMPE.test1();
//     }
//
//     #[test]
//     fn example1() {
//         EXAMPE.test1();
//     }
//
//     #[test]
//     fn example2() {
//         let input =
//             "
//                 1abc2
//                 pqr3stu8vwx
//                 a1b2c3d4e5f
//                 treb7uchet
//             "
//     ;
//
//     assert_eq!(part1(input), 142);
//     }
// }
//
// elvish::example!()
//     .example1("-L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF
// ", 142)
//     .example1("
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet
//     ", 142)
//     .example2("
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet
//     ", 142)
//     .test()

// elvish::examples! {
//     part1("
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet
//     ") == 142,
//
//     part2(r"
//         two1nine
//         eightwothree
//         abcone2threexyz
//         xtwone3four
//         4nineeightseven2
//         zoneight234
//         7pqrstsixteen
//     ") == 281,
// }
