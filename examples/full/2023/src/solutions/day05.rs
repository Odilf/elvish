use rayon::{prelude::*, slice::ParallelSlice};
use std::cmp::Ordering;

elvish::day!(5);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl Range {
    pub fn destination(&self) -> std::ops::Range<i64> {
        self.destination_start..self.destination_start + self.length
    }

    pub fn source(&self) -> std::ops::Range<i64> {
        self.source_start..self.source_start + self.length
    }

    pub fn cmp(&self, key: i64) -> Ordering {

        let source = self.source();
        let before_end = key < source.end;
        let after_start = key >= source.start;

        match (before_end, after_start) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    ranges: Vec<Range>,
}

impl From<Vec<Range>> for Map {
    fn from(mut ranges: Vec<Range>) -> Self {
        ranges.sort_by_key(|range| range.source_start);
        Self { ranges }
    }
}

impl Map {
    pub fn get(&self, key: i64) -> i64 {
        let Ok(range_index) = self.ranges.binary_search_by(|range| range.cmp(key)) else {
            return key;
        };

        let position = key - self.ranges[range_index].source_start;
        self.ranges[range_index].destination().start + position
    }
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> Range
            = _ destination_start:number() _ source_start:number() _ length:number() _ {
                Range { destination_start, source_start, length }
            }

        rule _ = [' ' | '\n']*

        rule map() -> Map
            = ranges:range() ++ _ { Map::from(ranges) }

        pub rule whole() -> (Vec<i64>, Vec<Map>)
            = seeds:("seeds:" _ seeds:number() ++ _ { seeds }) _
                maps:(['a'..='z' | 'A'..='Z' | '-']+ _ "map:" _ map:map() _ { map })* {
                    (seeds, maps)
        }
    }
}

fn find_lowest_seed(seeds: impl ParallelIterator<Item = i64>, maps: &[Map]) -> Option<i64> {
    seeds
        .map(|seed| {
            let mut value = seed;
            for map in maps {
                value = map.get(value);
            }
            value
        })
        .min()
}

fn part1(input: &str) -> i64 {
    let (seeds, maps) = parser::whole(input).unwrap();

    find_lowest_seed(seeds.into_par_iter(), &maps).unwrap()
}

/// This implementation is very brute-forcey, but with the help of
/// rayon it does compute the proper solution in a bit under 10s on
/// my machine.
fn part2(input: &str) -> i64 {
    let (seed_ranges, maps) = parser::whole(input).unwrap();

    let seeds = seed_ranges.par_chunks(2).flat_map(|chunk| {
        let [start, length] = chunk.try_into().unwrap();

        start..start + length
    });

    find_lowest_seed(seeds, &maps).unwrap()
}

elvish::examples! {
    r"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    " => 35, 46,
}
