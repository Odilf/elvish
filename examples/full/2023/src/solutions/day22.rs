use std::collections::{HashMap, HashSet};

elvish::day!(22);

type Vec3 = glam::I64Vec3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    p1: Vec3,
    p2: Vec3,
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.p1
            .z
            .cmp(&other.p1.z)
            .then_with(|| self.p2.z.cmp(&other.p2.z))
    }
}

impl Cube {
    fn supports(&self, other: &Cube) -> bool {
        self.xy_overlaps(other) && self.p2.z + 1 == other.p1.z
    }

    fn xy_overlaps(&self, other: &Cube) -> bool {
        self.p1.x <= other.p2.x
            && self.p1.y <= other.p2.y
            && self.p2.x >= other.p1.x
            && self.p2.y >= other.p1.y
    }

    fn move_by(&mut self, delta: Vec3) {
        self.p1 += delta;
        self.p2 += delta;
    }
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule vec() -> Vec3
            = x:number() "," y:number() "," z:number() { Vec3::new(x, y, z) }

        pub rule cube() -> Cube
            = p1:vec() "~" p2:vec() { Cube { p1, p2 } }
    }
}

fn collapse(cubes: &mut Vec<Cube>) {
    cubes.sort_unstable();

    for i in 0..cubes.len() {
        let mut bottomest = 0;

        let top = cubes[i];

        for j in 0..i {
            let bottom = cubes[j];

            if top.xy_overlaps(&bottom) && bottom.p2.z < top.p1.z && bottom.p2.z > bottomest {
                bottomest = bottom.p2.z;
            }
        }

        let delta_z = top.p1.z - bottomest;
        cubes[i].move_by([0, 0, -delta_z + 1].into())
    }

    cubes.sort_unstable();
}

fn get_supports(cubes: &Vec<Cube>) -> [HashMap<Cube, Vec<Cube>>; 2] {
    let mut supported_by = HashMap::new();
    let mut supports = HashMap::new();

    for i in 0..cubes.len() {
        let bottom = cubes[i];
        for j in i + 1..cubes.len() {
            let top = cubes[j];

            if bottom.supports(&top) {
                supported_by
                    .entry(top)
                    .or_insert_with(Vec::new)
                    .push(bottom);
                supports.entry(bottom).or_insert_with(Vec::new).push(top);
            }
        }
    }

    [supported_by, supports]
}

fn part1(input: &str) -> i64 {
    let mut cubes = input
        .lines()
        .map(|line| parser::cube(line).unwrap())
        .collect::<Vec<_>>();

    collapse(&mut cubes);

    let [supported_by, _supports] = get_supports(&cubes);

    cubes
        .iter()
        .filter(|cube| {
            supported_by
                .values()
                .filter(|supports| supports.contains(cube))
                .all(|supports| supports.len() > 1)
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let mut cubes = input
        .lines()
        .map(|line| parser::cube(line).unwrap())
        .collect::<Vec<_>>();

    collapse(&mut cubes);

    let [supported_by, supports] = get_supports(&cubes);

    cubes
        .into_iter()
        .map(|cube| {
            topple(
                cube,
                &supports,
                &supported_by,
                &mut HashSet::from([cube]),
                &mut HashSet::new(),
            ) - 1
        })
        .sum()
}

fn topple(
    cube: Cube,
    supports: &HashMap<Cube, Vec<Cube>>,
    supported_by: &HashMap<Cube, Vec<Cube>>,
    toppled: &mut HashSet<Cube>,
    visited: &mut HashSet<Cube>,
) -> i64 {
    if !visited.insert(cube) {
        return 0;
    }

    let Some(dependants) = supports.get(&cube) else {
        return 1;
    };

    // Try toppling everything that depends on this cube
    let topples = dependants
        .iter()
        .filter(|dependant| {
            // If still supported, don't topple
            let still_supported = supported_by
                .get(&dependant)
                .map(|supports| supports.iter().any(|cube| !toppled.contains(cube)))
                .unwrap_or(true);

            !still_supported
        })
        .collect::<Vec<_>>();

    toppled.extend(topples.clone());

    topples
        .into_iter()
        .map(|next_topple| topple(*next_topple, supports, supported_by, toppled, visited))
        .sum::<i64>()
        + 1
}

elvish::examples! {
    "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    " => 5, 7,
}
