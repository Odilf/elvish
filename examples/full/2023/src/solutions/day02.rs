elvish::day!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Blue = 0,
    Red = 1,
    Green = 2,
}

impl Color {
    pub const fn max(self) -> usize {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

peg::parser! {
    grammar day2() for str {
        rule number() -> i32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule color() -> Color
            = "blue" { Color::Blue }
            / "red" { Color:: Red }
            / "green" { Color::Green }

        rule numbered_color() -> (i32, Color)
            = n:number() " " c:color() { (n, c) }

        rule round() -> Vec<(i32, Color)>
            = nc:numbered_color() ** ", " { nc }

        pub rule game() -> (i32, Vec<Vec<(i32, Color)>>)
            = "Game " n:number() ": " r:round() ** "; " { (n, r) }
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (id, game) = day2::game(line).unwrap();

            for round in game {
                for (count, color) in round {
                    if count > color.max() as i32 {
                        return None;
                    }
                }
            }

            Some(id)
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, game) = day2::game(line).unwrap();
            let mut maxes = [0; 3];

            for round in game {
                for (count, color) in round {
                    maxes[color as usize] = maxes[color as usize].max(count);
                }
            }

            maxes.iter().product::<i32>()
        })
        .sum()
}

// elvish::examples! {
//     let input = r"
//         Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//     ";
//
//     part1(input) == 8, 
//
//     part2(input) == 2286,
// }
