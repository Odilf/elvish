use rayon::prelude::*;

elvish::day!(18);

type Vec2 = glam::I64Vec2;

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule direction() -> Direction
            = "U" { Direction::Up }
            / "D" { Direction::Down }
            / "L" { Direction::Left }
            / "R" { Direction::Right }

        rule hex() -> i64
             = "#" n:$(['0'..='9' | 'a'..='f'] * <6>) {
                i64::from_str_radix(n, 16).unwrap()
            }

        pub rule instruction() -> Instruction
            = d:direction() " " n:number() " (" c:hex() ")" {
                Instruction { direction: d, distance: n, color: c }
            }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: i64,
    color: i64,
}

#[derive(Debug, Clone, Copy)]
struct Wall {
    origin: Vec2,
    direction: Vec2,
    length: i64,
}

impl Wall {
    pub fn contains(&self, position: &Vec2) -> bool {
        let delta = (*position - self.origin) * self.direction;
        if self.direction.x == 0 {
            position.x == self.origin.x && 0 <= delta.y && delta.y < self.length
        } else if self.direction.y == 0 {
            position.y == self.origin.y && 0 <= delta.x && delta.x < self.length
        } else {
            unreachable!()
        }
    }

    pub fn contains_y(&self, y: i64) -> bool {
        if self.direction.x == 0 {
            let delta = (y - self.origin.y) * self.direction.y;
            delta >= 0 && delta < self.length
        } else if self.direction.y == 0 {
            self.origin.y == y
        } else {
            unreachable!()
        }
    }

    pub fn range_x(&self) -> [i64; 2] {
        let far = self.origin.x + self.direction.x * (self.length - 1);
        [self.origin.x.min(far), self.origin.x.max(far)]
    }
}

struct Map {
    walls: Vec<Wall>,
}

impl Map {
    pub fn contains(&self, position: &Vec2) -> bool {
        self.walls.iter().any(|wall| wall.contains(position))
    }

    pub fn area(&mut self) -> i64 {
        let min_y = self.walls.iter().map(|p| p.origin.y).min().unwrap();
        let max_y = self.walls.iter().map(|p| p.origin.y).max().unwrap();

        self.walls.sort_unstable_by_key(|wall| wall.range_x()[0]);

        (min_y..=max_y)
            .into_par_iter()
            .map(|y| {
                let mut walls = self
                    .walls
                    .iter()
                    .filter(|wall| wall.contains_y(y))
                    .peekable();

                let mut next_wall = || {
                    let wall = walls.next()?;
                    let [start, end] = wall.range_x();

                    if let Some(next) = walls.peek() {
                        let vertical_exists = wall.direction.x == 0 || next.direction.x == 0;
                        let are_connected = wall.range_x()[1] + 1 == next.range_x()[0];

                        if vertical_exists && are_connected {
                            let [_, end_second] = next.range_x();

                            let first_goes_up = self.contains(&Vec2::new(start, y - 1));
                            let second_goes_up = self.contains(&Vec2::new(end_second, y - 1));

                            // If not LJ shape
                            if first_goes_up != second_goes_up {
                                walls.next().unwrap();

                                return Some([start, end_second]);
                            }
                        }
                    };

                    Some([start, end])
                };

                let mut count = 0;
                while let Some([start, _]) = next_wall() {
                    let [_, end] = next_wall().expect("Bounds should be closed");
                    count += end - start + 1;
                }

                count
            })
            .sum()
    }
}

impl FromIterator<Wall> for Map {
    fn from_iter<T: IntoIterator<Item = Wall>>(iter: T) -> Self {
        Self {
            walls: iter.into_iter().collect(),
        }
    }
}

fn part1(input: &str) -> i64 {
    let instructions = input.lines().map(|line| parser::instruction(line).unwrap());

    let mut position = Vec2::ZERO;
    instructions
        .map(|instruction| {
            let origin = position;
            position = position + instruction.direction.delta() * instruction.distance;
            Wall {
                origin,
                direction: instruction.direction.delta(),
                length: instruction.distance,
            }
        })
        .collect::<Map>()
        .area()
}

fn part2(input: &str) -> i64 {
    let instructions = input.lines().map(|line| parser::instruction(line).unwrap());

    let mut position = Vec2::ZERO;
    instructions
        .map(|instruction| {
            let direction = instruction.color % 16;
            let distance = instruction.color as i64 / 16;

            let delta = match direction {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => unreachable!(),
            }
            .delta();

            let origin = position;
            position += delta * distance as i64;

            Wall {
                origin,
                direction: delta,
                length: distance,
            }
        })
        .collect::<Map>()
        .area()
}

elvish::examples! {
    "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    " => 62, 952_408_144_115,
}
