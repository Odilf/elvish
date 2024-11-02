use std::collections::HashMap;

elvish::day!(4);

peg::parser! {
    grammar parser() for str {
        rule number() -> i32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule card() -> i32
            = n:number() { n }

        rule _() = [' ' | '\n']+

        rule cards() -> Vec<i32>
            = cards:(card() ** _) { cards }

        pub rule line() -> [Vec<i32>; 2]
            = "Card" _ number() ":" _ winning:cards() _ "|" _ cards:cards() { [winning, cards] }
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parser::line(line).unwrap())
        .map(|[winning, cards]| {
            let matches = cards.iter().filter(|&card| winning.contains(card)).count();

            if matches == 0 {
                0
            } else {
                2_i32.pow(matches as u32 - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    let mut copies = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let [winning, cards] = parser::line(line).unwrap();
        let copies_current = copies.get(&i).unwrap_or(&0) + 1;
        count += copies_current;

        let matches = cards.iter().filter(|&card| winning.contains(card)).count();
        for next in i + 1..=i + matches {
            copies.insert(next, copies.get(&next).unwrap_or(&0) + copies_current);
        }
    }

    count
}

// elvish::examples! {
//     let input = r"
//         Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
//         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
//         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
//         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
//         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
//         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
//     ";
//
//     part1(input) == 13, 
//     part2(input) == 30,
// }
