use std::collections::HashMap;

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe { vertical: bool },
    Bend { north: bool, east: bool },
}

enum NonTile {
    Empty,
    Start,
}

pub const NORTH: IVec2 = IVec2::new(0, -1);
pub const SOUTH: IVec2 = IVec2::new(0, 1);
pub const EAST: IVec2 = IVec2::new(1, 0);
pub const WEST: IVec2 = IVec2::new(-1, 0);

impl TryFrom<char> for Tile {
    type Error = NonTile;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Tile::*;

        Ok(match c {
            '|' => Pipe { vertical: true },
            '-' => Pipe { vertical: false },
            'L' => Bend {
                north: true,
                east: true,
            },
            'J' => Bend {
                north: true,
                east: false,
            },
            '7' => Bend {
                north: false,
                east: false,
            },
            'F' => Bend {
                north: false,
                east: true,
            },
            '.' => return Err(NonTile::Empty),
            'S' => return Err(NonTile::Start),
            _ => panic!("Invalid tile: {}", c),
        })
    }
}

impl Tile {
    pub const fn all_neighbor_deltas() -> [IVec2; 4] {
        [NORTH, SOUTH, EAST, WEST]
    }

    pub fn neighbor_deltas(&self) -> [IVec2; 2] {
        match self {
            Self::Pipe { vertical: true } => [NORTH, SOUTH],
            Self::Pipe { vertical: false } => [EAST, WEST],
            Self::Bend {
                north: true,
                east: true,
            } => [NORTH, EAST],
            Self::Bend {
                north: true,
                east: false,
            } => [NORTH, WEST],
            Self::Bend {
                north: false,
                east: false,
            } => [SOUTH, WEST],
            Self::Bend {
                north: false,
                east: true,
            } => [SOUTH, EAST],
        }
        .map(Into::into)
    }

    pub const fn north(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => *vertical,
            Tile::Bend { north, .. } => *north,
        }
    }

    pub const fn south(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => *vertical,
            Tile::Bend { north, .. } => !*north,
        }
    }

    pub const fn east(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => !*vertical,
            Tile::Bend { east, .. } => *east,
        }
    }

    pub const fn west(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => !*vertical,
            Tile::Bend { east, .. } => !*east,
        }
    }
}

fn parse(input: &str) -> (HashMap<IVec2, Tile>, IVec2) {
    let mut output = HashMap::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            match Tile::try_from(c) {
                Ok(tile) => {
                    output.insert(pos, tile);
                }

                Err(NonTile::Start) => {
                    assert!(start.is_none());
                    start = Some(pos);
                }

                Err(NonTile::Empty) => (),
            };
        }
    }

    let start = start.unwrap();

    set_start(&mut output, start);

    (output, start)
}

fn set_start(map: &mut HashMap<IVec2, Tile>, start: IVec2) {
    let start_is_north = map
        .get(&(start + NORTH))
        .map(|tile| tile.south())
        .unwrap_or(false);

    let start_is_east = map
        .get(&(start + EAST))
        .map(|tile| tile.west())
        .unwrap_or(false);

    map.insert(
        start,
        Tile::Bend {
            north: start_is_north,
            east: start_is_east,
        },
    );
}

fn get_loop(map: &HashMap<IVec2, Tile>, start: IVec2) -> HashMap<IVec2, Tile> {
    let mut output = HashMap::new();

    let mut pos = start;

    loop {
        let tile = map.get(&pos).unwrap();

        output.insert(pos, *tile);

        let mut found = false;

        for delta in tile.neighbor_deltas() {
            if output.contains_key(&(pos + delta)) {
                continue;
            }

            let neighbor = pos + delta;

            if map.contains_key(&neighbor) {
                pos = neighbor;
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }

    output
}

#[elvish::solution(day=10, example=[4, 4, 8])]
fn part1(input: &str) -> i64 {
    let (map, start) = parse(input);

    let main_loop = get_loop(&map, start);

    main_loop.len() as i64 / 2
}

#[elvish::solution(day=10, example=[4, 4, 8, 10])]
fn part2(input: &str) -> i64 {
    let (map, start) = parse(input);

    let size = IVec2::new(
        map.keys().map(|pos| pos.x).max().unwrap(),
        map.keys().map(|pos| pos.y).max().unwrap(),
    );

    let main_loop = get_loop(&map, start);

    let mut count = 0;

    for y in 0..=size.y {
        let mut inside = false;

        for x in 0..=size.x {
            let pos = IVec2::new(x, y);

            match main_loop.get(&pos) {
                Some(Tile::Pipe { vertical: true } | Tile::Bend { north: true, .. }) => {
                    inside = !inside
                }

                None if inside => {
                    count += 1;
                }

                _ => (),
            }
        }
    }

    count
}

elvish::example!(
    part1: "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    ",

    part1: "
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    ",

    part1: "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    ",


    part2: "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    ",

    part2: "
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    ",

    part2: "
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    ",

    part2: "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    ",
);
