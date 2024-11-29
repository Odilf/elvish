use std::collections::HashSet;

type Vec2 = glam::I64Vec2;

fn parse(input: &str, expansion_multiplier: i64) -> Vec<Vec2> {
    let expansion_size = expansion_multiplier - 1;

    let size = Vec2::new(
        input.lines().next().unwrap().len() as i64,
        input.lines().count() as i64,
    );

    let input = input.as_bytes();

    let index = |x, y| input[(y * (size.x + 1) + x) as usize];

    let empty_columnns = (0..size.x)
        .filter(|&x| (0..size.y).all(|y| index(x, y) == b'.'))
        .collect::<HashSet<_>>();

    let mut output = Vec::new();
    let mut offset = Vec2::ZERO;
    for y in 0..size.y {
        offset.x = 0;
        let mut is_column_empty = true;

        for x in 0..size.x {
            if empty_columnns.contains(&x) {
                offset.x += expansion_size;
            } else if index(x, y) == b'#' {
                output.push(Vec2::new(x, y) + offset);
                is_column_empty = false;
            }
        }

        if is_column_empty {
            offset.y += expansion_size;
        }
    }

    output
}

fn solve(input: &str, expansion_multiplier: i64) -> i64 {
    let galaxies = parse(input, expansion_multiplier);

    let mut count = 0;

    for (i, &galaxy) in galaxies.iter().enumerate() {
        for &other in galaxies[i + 1..].iter() {
            let Vec2 { x, y } = (galaxy - other).abs();
            count += x + y;
        }
    }

    count
}

#[elvish::solution(day = 11, example = 374)]
fn part1(input: &str) -> i64 {
    solve(input, 2)
}

// Got the second example myself, because given example has different parameters
#[elvish::solution(day = 11, example = 82000210)]
fn part2(input: &str) -> i64 {
    solve(input, 1_000_000)
}

elvish::example!(
    "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "
);
