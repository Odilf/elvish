use std::collections::{HashMap, HashSet};

use either::Either;
use glam::I64Vec2;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Splitter {
        vertical: bool,
    },
    /// Forward is `/`, not foward is `\`
    Mirror {
        forward: bool,
    },
}

impl Tile {
    pub fn reflect(&self, beam: Beam) -> impl Iterator<Item = Beam> {
        let result = match self {
            Tile::Mirror { forward } => Beam {
                vertical: !beam.vertical,
                positive: *forward != beam.positive,
            },
            Tile::Splitter { vertical } if *vertical == beam.vertical => beam,
            &Tile::Splitter { vertical } => {
                return Either::Right(
                    [Beam::new(vertical, true), Beam::new(vertical, false)].into_iter(),
                )
            }
        };

        Either::Left(std::iter::once(result))
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Beam {
    vertical: bool,

    /// Vertical positive is down,
    positive: bool,
}

impl Beam {
    pub fn new(vertical: bool, positive: bool) -> Self {
        Self { vertical, positive }
    }

    pub fn delta(&self) -> I64Vec2 {
        let dir = if self.positive { 1 } else { -1 };
        if self.vertical { [0, dir] } else { [dir, 0] }.into()
    }
}

fn parse(input: &str) -> HashMap<I64Vec2, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let pos = I64Vec2::new(x as i64, y as i64);
                let tile = match c {
                    '|' => Tile::Splitter { vertical: true },
                    '-' => Tile::Splitter { vertical: false },
                    '/' => Tile::Mirror { forward: true },
                    '\\' => Tile::Mirror { forward: false },
                    '.' => return None,
                    _ => panic!("Invalid tile ({c})"),
                };

                Some((pos, tile))
            })
        })
        .collect()
}

fn energize(start: I64Vec2, beam: Beam, map: &HashMap<I64Vec2, Tile>) -> HashSet<I64Vec2> {
    let mut energized = HashSet::new();
    let mut history = HashSet::new();

    let map_size = I64Vec2 {
        x: map.keys().map(|pos| pos.x).max().unwrap(),
        y: map.keys().map(|pos| pos.y).max().unwrap(),
    };

    let mut queue = vec![(start, beam)];

    while let Some((pos, beam)) = queue.pop() {
        if !history.insert((pos, beam))
            || pos.x < 0
            || pos.y < 0
            || pos.x > map_size.x
            || pos.y > map_size.y
        {
            continue;
        }

        energized.insert(pos);

        let mut schedule_next = |beam: Beam| queue.push((pos + beam.delta(), beam));

        if let Some(tile) = map.get(&pos) {
            for beam in tile.reflect(beam) {
                schedule_next(beam);
            }
        } else {
            schedule_next(beam);
        }
    }

    energized
}

#[elvish::solution(day = 16, example = 46)]
fn part1(input: &str) -> i64 {
    let map = parse(input);

    energize(
        [0, 0].into(),
        Beam {
            vertical: false,
            positive: true,
        },
        &map,
    )
    .len() as i64
}

#[elvish::solution(day = 16, example = 51)]
fn part2(input: &str) -> i64 {
    let map = parse(input);
    let map_size = I64Vec2 {
        x: map.keys().map(|pos| pos.x).max().unwrap(),
        y: map.keys().map(|pos| pos.y).max().unwrap(),
    };

    (0..=map_size.x)
        .into_par_iter()
        .flat_map(|x| {
            [
                ([x, 0], Beam::new(true, true)),
                ([x, map_size.y], Beam::new(true, false)),
            ]
        })
        .chain((0..=map_size.y).into_par_iter().flat_map(|y| {
            [
                ([0, y], Beam::new(false, true)),
                ([map_size.x, y], Beam::new(false, false)),
            ]
        }))
        .map(|(pos, beam)| energize(pos.into(), beam, &map).len())
        .max()
        .unwrap() as i64
}

elvish::example!(
    r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "
);
