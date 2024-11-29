use std::collections::HashMap;

use rayon::{prelude::ParallelIterator, str::ParallelString};

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule tile() -> Spring
            = "#" { Spring::Operational }
            / "?" { Spring::Unknown }
            / "." { Spring::Damaged }

        rule _ = [' ' | '\t' | '\n']*

        pub rule line() -> (Vec<Spring>, Vec<i64>)
            = tiles:tile()+ _ numbers:number() ** "," { (tiles, numbers) }

        pub rule lines() -> Vec<(Vec<Spring>, Vec<i64>)>
            = l:line() ** "\n" { l }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BundleData {
    index: usize,
    used: i64,
    last_was_operational: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Data {
    spring_index: usize,
    bundle: BundleData,
}

impl Data {
    pub fn next(&self) -> Self {
        Self {
            spring_index: self.spring_index + 1,
            ..self.clone()
        }
    }

    pub fn use_bundle(&self) -> Self {
        Self {
            spring_index: self.spring_index + 1,
            bundle: BundleData {
                used: self.bundle.used + 1,
                last_was_operational: true,
                ..self.bundle
            },
        }
    }

    pub fn next_bundle(&self) -> Self {
        Self {
            spring_index: self.spring_index + 1,
            bundle: BundleData {
                used: 0,
                index: self.bundle.index + 1,
                last_was_operational: false,
            },
        }
    }
}

fn count_arrangments(springs: &[Spring], bundles: &[i64]) -> i64 {
    fn execute(
        data: Data,
        springs: &[Spring],
        bundles: &[i64],
        cache: &mut HashMap<Data, i64>,
    ) -> i64 {
        if let Some(&count) = cache.get(&data) {
            return count;
        }

        let Data {
            spring_index,
            bundle,
        } = data;

        let spring = springs.get(spring_index);
        let bundle_size = bundles
            .get(bundle.index)
            .map(|b| b - bundle.used)
            .filter(|b| *b > 0);

        let exec_operational = |cache| match bundle_size {
            None => 0,
            Some(_) => execute(data.use_bundle(), springs, bundles, cache),
        };

        let exec_damaged = |cache| match bundle_size {
            None => execute(data.next_bundle(), springs, bundles, cache),
            Some(_) => {
                if data.bundle.last_was_operational {
                    0
                } else {
                    execute(data.next(), springs, bundles, cache)
                }
            }
        };

        let result = match spring {
            None => match bundle_size {
                None if bundle.index >= bundles.len()
                    || (bundle.index == bundles.len() - 1
                        && bundles[bundle.index] == bundle.used) =>
                {
                    1
                }
                _ => 0,
            },

            Some(spring) => match spring {
                Spring::Operational => exec_operational(cache),
                Spring::Damaged => exec_damaged(cache),
                Spring::Unknown => exec_operational(cache) + exec_damaged(cache),
            },
        };

        cache.insert(data, result);
        result
    }

    execute(Data::default(), springs, bundles, &mut HashMap::new())
}

#[elvish::solution(day = 12, example = 21)]
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (beacons, bundles) = parser::line(line).unwrap();
            count_arrangments(&beacons, &bundles)
        })
        .sum()
}

#[elvish::solution(day = 12, example = 525152)]
fn part2(input: &str) -> i64 {
    input
        .par_lines()
        .map(|line| {
            let (beacons, bundles) = parser::line(line).unwrap();
            let beacons = vec![beacons; 5].join(&Spring::Unknown);
            let bundles = std::iter::repeat(bundles)
                .take(5)
                .flatten()
                .collect::<Vec<_>>();

            count_arrangments(&beacons, &bundles)
        })
        .sum()
}

elvish::example!(
    "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "
);
