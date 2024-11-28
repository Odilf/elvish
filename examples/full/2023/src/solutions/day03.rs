use glam::i32::IVec2;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct NumberData {
    value: i32,
    length: i32,
    index: i32,
}

impl NumberData {
    pub fn start(&self, coords: IVec2) -> IVec2 {
        coords - IVec2::X * self.index
    }
}

fn parse(
    input: &str,
    symbol_predicate: impl Fn(char) -> bool,
) -> (HashMap<IVec2, NumberData>, HashSet<IVec2>) {
    let mut numbers = HashMap::<IVec2, NumberData>::new();
    let mut symbols = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coords = IVec2::new(x as i32, y as i32);

            if let Some(digit) = c.to_digit(10) {
                let digit = digit as i32;
                let coords_prev = coords - IVec2::X;
                if let Some(&prev) = numbers.get(&coords_prev) {
                    let value = prev.value * 10 + digit;
                    for i in 0..=prev.length {
                        numbers.insert(
                            prev.start(coords_prev) + IVec2::X * i,
                            NumberData {
                                value,
                                length: prev.length + 1,
                                index: i,
                            },
                        );
                    }
                } else {
                    numbers.insert(
                        coords,
                        NumberData {
                            value: digit,
                            length: 1,
                            index: 0,
                        },
                    );
                }
            } else if symbol_predicate(c) {
                symbols.insert(coords);
            }
        }
    }

    (numbers, symbols)
}

fn get_and_remove(numbers: &mut HashMap<IVec2, NumberData>, coords: IVec2) -> Option<i32> {
    let number = numbers.remove(&coords)?;

    for i in 0..number.length {
        if i == number.index {
            continue;
        }

        let removed = numbers.remove(&(number.start(coords) + IVec2::X * i));
        debug_assert!(removed.is_some());
    }

    Some(number.value)
}

fn neighbor_offsets() -> impl Iterator<Item = IVec2> {
    (-1i32..=1)
        .flat_map(|dy| (-1i32..=1).map(move |dx| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .map(|(dx, dy)| IVec2::new(dx, dy))
}

#[elvish::solution(day=3, example=4361)]
fn part1(input: &str) -> i32 {
    let (mut numbers, symbols) = parse(input, |c| c != '.');

    let mut count = 0i32;
    for coords in symbols {
        for delta in neighbor_offsets() {
            count += get_and_remove(&mut numbers, coords + delta).unwrap_or(0);
        }
    }

    count
}

fn get_exacly_two(mut iter: impl Iterator<Item = i32>) -> Option<[i32; 2]> {
    let first = iter.next()?;
    let second = iter.next()?;
    let None = iter.next() else {
        return None;
    };

    Some([first, second])
}

#[elvish::solution(day=3, example=467835)]
fn part2(input: &str) -> i32 {
    let (mut numbers, symbols) = parse(input, |c| c == '*');

    let mut output = 0;
    for coords in symbols {
        let iter =
            neighbor_offsets().filter_map(|delta| get_and_remove(&mut numbers, delta + coords));

        if let Some(nums) = get_exacly_two(iter) {
            let gear_ratio = nums.iter().product::<i32>();
            output += gear_ratio;
        }
    }

    output
}

elvish::example!("
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
");
