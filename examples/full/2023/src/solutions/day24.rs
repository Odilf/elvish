use nalgebra::{vector, Matrix3, Matrix6};

elvish::day!(24);

type Vec3 = nalgebra::Vector3<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line<T = Vec3> {
    pos: T,
    vel: T,
}

impl<T: Copy> Line<T> {
    fn map<R, F: Fn(T) -> R>(&self, f: F) -> Line<R> {
        Line {
            pos: f(self.pos),
            vel: f(self.vel),
        }
    }
}

peg::parser! {
    grammar parser() for str {
        rule _ = [' ' | '\n']*

        rule number() -> i64
            = n:$(['-' | '0'..='9']+) { n.parse().unwrap() }

        pub rule vector() -> Vec3
            = x:number() "," _ y:number() "," _ z:number() { Vec3::new(x, y, z) }

        pub rule line() -> Line
            = pos:vector() _ "@" _ vel:vector() { Line { pos, vel } }
    }
}

fn part1(input: &str) -> i64 {
    solve1(input, 200_000_000_000_000, 400_000_000_000_000)
}

fn solve1(input: &str, min: i64, max: i64) -> i64 {
    let lines = input
        .lines()
        .map(|line| parser::line(line).unwrap())
        .collect::<Vec<_>>();

    let min = min as f32;
    let max = max as f32;

    let mut count = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let (a, b) = (&lines[i], &lines[j]);

            // TODO: Removing the last column seems to break for some reason...
            let matrix = Matrix3::from_columns(&[a.vel, -b.vel, Vec3::z()]).cast::<f32>();

            let Some(inverse) = matrix.try_inverse() else {
                continue;
            };

            let lambdas = inverse * (b.pos - a.pos).cast();

            if lambdas.x < 0.0 || lambdas.y < 0.0 {
                continue;
            }

            let intersection = a.pos.cast() + a.vel.cast() * lambdas.x;

            if intersection.xy().iter().all(|&x| min < x && x < max) {
                count += 1;
            }
        }
    }

    count
}

fn god_throw(lines: [Line; 3]) -> Option<(Vec3, Vec3)> {
    let matrices = [1, 2].map(|i| {
        Line {
            pos: lines[0].pos - lines[i].pos,
            vel: lines[0].vel - lines[i].vel,
        }
        .map(|v| v.cast::<f64>().cross_matrix())
    });

    let mut matrix = Matrix6::zeros();

    matrix.index_mut((0..3, 0..3)).copy_from(&-matrices[0].vel);
    matrix.index_mut((3..6, 0..3)).copy_from(&-matrices[1].vel);

    matrix.index_mut((0..3, 3..6)).copy_from(&matrices[0].pos);
    matrix.index_mut((3..6, 3..6)).copy_from(&matrices[1].pos);

    let inverse = matrix.try_inverse().unwrap();

    let crosses = lines.map(|line| line.pos.cross(&line.vel));

    let v1 = crosses[0] - crosses[1];
    let v2 = crosses[0] - crosses[2];

    let v = vector![v1.x, v1.y, v1.z, v2.x, v2.y, v2.z];

    let result = inverse * v.cast();

    if result.iter().any(|x| (x.round() - x).abs() > 0.0000001) {
        return None;
    }

    let position = result.fixed_view::<3, 1>(0, 0).map(|v| v.round() as i64);
    let velocity = result.fixed_view::<3, 1>(3, 0).map(|v| v.round() as i64);

    Some((position, velocity))
}

fn part2(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| parser::line(line).unwrap())
        .collect::<Vec<_>>();

    let half_cartesian = {
        let n = lines.len();
        (0..n).flat_map(move |i| (i + 1..n).flat_map(move |j| (j + 1..n).map(move |k| (i, j, k))))
    };

    for (i, j, k) in half_cartesian {
        let lines = [lines[i], lines[j], lines[k]];

        let Some((position, velocity)) = god_throw(lines) else {
            continue;
        };

        // Check for integer collision times
        let valid = lines.iter().all(|line| {
            (0..3).all(|i| {
                let denom = line.vel[i] - velocity[i];
                denom == 0 || (position[i] - line.pos[i]) % denom == 0
            })
        });

        if valid {
            return position.iter().sum();
        }
    }

    panic!("No solution found");
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = elvish::indoc! {
        "
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "
    };

    #[test]
    fn part1() {
        assert_eq!(solve1(TEST_INPUT, 7, 27), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT), 47);
    }
}
