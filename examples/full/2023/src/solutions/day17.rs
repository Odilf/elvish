use std::collections::{BinaryHeap, HashMap};

use glam::I64Vec2;
use ndarray::{Array1, Array2};

elvish::day!(17);

type Vec2 = I64Vec2;

fn parse(input: &str) -> Array2<i64> {
    let map = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i64))
        .collect::<Array1<_>>();

    let size = (input.lines().count(), input.lines().next().unwrap().len());

    map.into_shape(size).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: i64,
    position: I64Vec2,
    direction: I64Vec2,
    moved_straight: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Key = (I64Vec2, I64Vec2, i64);

impl Node {
    pub fn key(&self) -> Key {
        (self.position, self.direction, self.moved_straight)
    }
}

fn solve(map: &Array2<i64>, min_straight: i64, max_straight: i64) -> i64 {
    let size = map.shape();

    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    queue.push(Node {
        cost: 0,
        position: Vec2::ZERO,
        direction: Vec2::X,
        moved_straight: 0,
    });

    while let Some(node) = queue.pop() {
        let Node {
            cost,
            position,
            direction,
            moved_straight,
        } = &node;

        if [position.y as usize + 1, position.x as usize + 1] == size
            && *moved_straight >= min_straight
        {
            return *cost;
        }

        let is_better = |node: &Node, costs: &HashMap<Key, i64>| {
            costs
                .get(&node.key())
                .map(|&prev_cost| node.cost < prev_cost)
                .unwrap_or(true)
        };

        if !is_better(&node, &costs) {
            continue;
        }

        if Some(cost) < costs.get(&node.key()) {
            continue;
        }

        costs.insert(node.key(), *cost);

        let mut insert = |direction, moved_straight| {
            let position: Vec2 = *position + direction;

            let Some(map_value) = map.get((position[1] as usize, position[0] as usize)) else {
                return;
            };

            let cost = cost + map_value;

            let next = Node {
                cost,
                position,
                direction,
                moved_straight,
            };

            if is_better(&next, &costs) {
                queue.push(next);
            }
        };

        if *moved_straight >= min_straight {
            let left = Vec2::new(direction.y, -direction.x);
            let right = Vec2::new(-direction.y, direction.x);

            insert(right, 1);
            insert(left, 1);
        }

        if *moved_straight < max_straight {
            insert(*direction, moved_straight + 1);
        }
    }

    panic!("No solution found");
}

fn part1(input: &str) -> i64 {
    let map = parse(input);

    solve(&map, 0, 3)
}

fn part2(input: &str) -> i64 {
    let map = parse(input);

    solve(&map, 4, 10)
}

elvish::examples! {
    "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    " => 102, 94,
}

#[test]
fn part2_2() {
    let input = elvish::indoc! { "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    " };

    assert_eq!(part2(input), 71);
}
