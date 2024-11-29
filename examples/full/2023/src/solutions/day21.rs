use std::{collections::HashSet, mem};

type Vec2 = glam::I64Vec2;

fn parse(input: &str) -> (HashSet<Vec2>, Vec2, i64) {
    let mut start = None;
    let mut size = Vec2::ZERO;

    let mut walls = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        size.y = size.y.max(y as i64);
        for (x, c) in line.chars().enumerate() {
            size.x = size.x.max(x as i64);

            let pos = Vec2::new(x as i64, y as i64);

            if c == '#' {
                walls.insert(pos);
            } else if c == 'S' {
                assert!(start.is_none());
                start = Some(pos);
            }
        }
    }

    assert_eq!(size.x, size.y);

    let start = start.unwrap();

    (walls, start, size.x + 1)
}

const DIRECTIONS: [Vec2; 4] = [
    Vec2::new(0, -1),
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(-1, 0),
];

#[elvish::solution(day = 21)]
fn part1(input: &str) -> i64 {
    solve1(input, 64)
}

fn solve1(input: &str, steps: i64) -> i64 {
    let (walls, start, size) = parse(input);

    count_locations(&walls, start, steps, size)
}

fn count_locations(walls: &HashSet<Vec2>, start: Vec2, steps: i64, size: i64) -> i64 {
    let mut queue = vec![start];
    let mut visited = HashSet::new();
    let mut count = 0;

    for i in 0..=steps {
        let mut next_queue = Vec::new();

        while let Some(node) = queue.pop() {
            let original = Vec2::new(node.x.rem_euclid(size), node.y.rem_euclid(size));
            if walls.contains(&original) || !visited.insert(node) {
                continue;
            }

            if i % 2 == steps % 2 {
                count += 1;
            }

            for dir in DIRECTIONS.iter() {
                let next = node + *dir;
                next_queue.push(next);
            }
        }

        mem::swap(&mut queue, &mut next_queue);
    }

    count
}

#[elvish::solution(day = 21)]
fn part2(input: &str) -> i64 {
    const STEPS: i64 = 26_501_365;

    let (walls, start, size) = parse(input);

    let x = [0, 1, 2].map(|i| (i * 2 + 1) * size / 2);
    let y = x.map(|x| count_locations(&walls, start, x, size) as f64);

    // Lagrange polynomial
    let result = (0..3)
        .map(|i| {
            (0..3)
                .filter(|&j| i != j)
                .map(|j| {
                    let num = STEPS - x[j];
                    let den = x[i] - x[j];

                    num as f64 / den as f64
                })
                .product::<f64>()
                * y[i]
        })
        .sum::<f64>();

    assert!((result.floor() - result).abs() <= 0.001);

    result.floor() as i64
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = elvish::indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn part1() {
        assert_eq!(super::solve1(TEST_INPUT, 6), 16);
    }
}
