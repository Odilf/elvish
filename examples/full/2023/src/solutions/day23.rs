use std::collections::{HashMap, HashSet};

type Vec2 = glam::I64Vec2;

enum Tile {
    Path,
    Slope(Vec2),
}

const DIRECTIONS: [Vec2; 4] = [
    Vec2::new(0, -1),
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(-1, 0),
];

fn parse(input: &str) -> HashMap<Vec2, Tile> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2::new(x as i64, y as i64);

            let tile = match c {
                '.' => Tile::Path,
                '>' => Tile::Slope(Vec2::X),
                'v' => Tile::Slope(Vec2::Y),
                '<' => Tile::Slope(-Vec2::X),
                '^' => Tile::Slope(-Vec2::Y),
                '#' => continue,
                _ => panic!("invalid tile ({c})"),
            };

            map.insert(pos, tile);
        }
    }

    map
}

#[elvish::solution(day = 23, example=94)]
fn part1(input: &str) -> i64 {
    let map = parse(input);

    let size = map
        .keys()
        .fold(Vec2::ZERO, |size, pos| size.max(*pos + Vec2::ONE));

    longest_path(
        Vec2::new(1, 0),
        &map,
        &mut HashMap::new(),
        &mut HashSet::new(),
        &(size + Vec2::new(-1, -1)),
    )
    .unwrap()
}

fn longest_path(
    position: Vec2,
    map: &HashMap<Vec2, Tile>,
    cache: &mut HashMap<Vec2, i64>,
    visited: &mut HashSet<Vec2>,
    goal: &Vec2,
) -> Option<i64> {
    if let Some(&result) = cache.get(&position) {
        return Some(result);
    }

    if !visited.insert(position) {
        return None;
    }

    if position == *goal {
        return Some(0);
    }

    let result = DIRECTIONS
        .into_iter()
        .filter_map(|dir| {
            let next_pos = position + dir;
            let tile = map.get(&next_pos)?;

            match tile {
                Tile::Slope(slope) if dir != *slope => return None,
                _ => (),
            };

            longest_path(next_pos, map, cache, visited, goal)
        })
        .map(|d| d + 1)
        .max()
        .unwrap();

    cache.insert(position, result);

    Some(result)
}

fn build_graph(
    map: &HashMap<Vec2, Tile>,
    start: Vec2,
    goal: Vec2,
) -> (
    HashSet<Vec2>,
    HashSet<(Vec2, i64, Vec2)>,
    HashMap<Vec2, Vec<(Vec2, i64)>>,
) {
    let mut queue = vec![(start, 0, start)];
    let mut visited = HashSet::new();

    let mut nodes = HashSet::from([start, goal]);
    let mut edges = HashSet::new();

    while let Some((previous_node, distance, position)) = queue.pop() {
        if previous_node == position && position != start {
            continue;
        }

        if nodes.contains(&position) {
            edges.insert((previous_node, distance, position));
        }

        // We check this condition after because we want to add edges
        // to visited nodes (but not explore them further).
        if !visited.insert(position) {
            continue;
        }

        let neighbors = DIRECTIONS
            .into_iter()
            .map(|dir| position + dir)
            .filter(|pos| map.get(pos).is_some())
            .collect::<Vec<_>>();

        let (previous, distance) = if neighbors.len() > 2 {
            nodes.insert(position);
            edges.insert((previous_node, distance, position));

            (position, 1)
        } else {
            (previous_node, distance + 1)
        };

        for neighbor in neighbors {
            queue.push((previous, distance, neighbor));
        }
    }

    let mut neighbors = HashMap::new();

    for &(a, distance, b) in &edges {
        for (from, to) in [(a, b), (b, a)] {
            neighbors
                .entry(from)
                .or_insert_with(Vec::new)
                .push((to, distance));
        }
    }

    (nodes, edges, neighbors)
}

#[elvish::solution(day = 23, example=154)]
fn part2(input: &str) -> i64 {
    let map = parse(input);

    let size = map
        .keys()
        .fold(Vec2::ZERO, |size, pos| size.max(*pos + Vec2::ONE));

    let start = Vec2::new(1, 0);
    let goal = size - Vec2::ONE;

    let (_, _, neighbors) = build_graph(&map, start, goal);

    longest_graph_path(start, &neighbors, &mut Vec::new(), &goal).unwrap()
}

fn longest_graph_path(
    node: Vec2,
    neighbors: &HashMap<Vec2, Vec<(Vec2, i64)>>,
    path: &mut Vec<Vec2>,
    goal: &Vec2,
) -> Option<i64> {
    if path.contains(&node) {
        return None;
    }

    if node == *goal {
        return Some(0);
    }

    path.push(node);

    let result = &neighbors[&node]
        .iter()
        .filter_map(|&(neighbor, distance)| {
            longest_graph_path(neighbor, neighbors, path, goal).map(|d| d + distance)
        })
        .max();

    path.pop();

    *result
}

elvish::example!(
    "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "
);
