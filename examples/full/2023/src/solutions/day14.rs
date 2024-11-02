use ndarray::{Array1, Array2, Axis};

elvish::day!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

fn parse(input: &str) -> Array2<Option<Rock>> {
    let size = (
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );

    let array = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'O' => Some(Rock::Round),
                '#' => Some(Rock::Cube),
                '.' => None,
                _ => panic!("Invalid input ({c})"),
            })
        })
        .collect::<Array1<_>>();

    array.into_shape(size).unwrap()
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt<'a>(grid: &mut Array2<Option<Rock>>, direction: Direction) {
    let (lanes, reversed) = match direction {
        Direction::North => (grid.columns_mut(), false),
        Direction::South => (grid.columns_mut(), true),
        Direction::West => (grid.rows_mut(), false),
        Direction::East => (grid.rows_mut(), true),
    };

    for mut row in lanes {
        let mut spot_available = if reversed { row.len() - 1 } else { 0 } as i64;

        let range = 0..row.len();

        let delta = if reversed { -1 } else { 1 };

        let callback = |i| {
            let rock = &row[i];

            match rock {
                Some(Rock::Round) => {
                    row[i] = None;
                    row[spot_available as usize] = Some(Rock::Round);
                    spot_available += delta;
                }
                Some(Rock::Cube) => {
                    spot_available = i as i64 + delta;
                }
                None => (),
            }
        };

        if reversed {
            (range.rev()).for_each(callback);
        } else {
            range.for_each(callback)
        }
    }
}

fn north_stress(grid: &Array2<Option<Rock>>) -> i64 {
    grid.rows()
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let value = (grid.len_of(Axis(0)) - i) as i64;
            row.iter()
                .filter(|rock| matches!(rock, Some(Rock::Round)))
                .count() as i64
                * value
        })
        .sum()
}

fn part1(input: &str) -> i64 {
    let mut grid = parse(input);

    tilt(&mut grid, Direction::North);

    north_stress(&grid)
}

fn part2(input: &str) -> i64 {
    let mut grid = parse(input);
    let mut history = Vec::new();

    const ITERATIONS: usize = 1_000_000_000;

    for i in 1..=ITERATIONS {
        history.push(grid.clone());

        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            tilt(&mut grid, dir);
        }

        if let Some(loop_start) = history.iter().position(|g| g == &grid) {
            let loop_size = i - loop_start;
            let remaining = ITERATIONS - i;
            let index = remaining % loop_size + loop_start;

            assert!(loop_start <= index && index < i);

            return north_stress(&history[index]);
        }

    }

    north_stress(&grid)
}

elvish::examples! {
    r"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    " => 136, 64,
}
