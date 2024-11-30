//! Traits for declaring solutions of advent of code puzzles, and functions for running them. 

use std::fmt::Display;

/// A solution of a part of an advent of code puzzle. 
///
/// It takes the input as a string slice and returns some output that can be converted to a string.
pub trait Part<const PART: u8, const DAY: u8> {
    /// Solves the puzzle for that part. 
    fn solve(input: &str) -> impl Display;
}

/// Solution for both parts of a day 
///
/// It is auto-implemented for any type that implements [`Part`] for both parts of
/// a day. 
pub trait Day<const DAY: u8>: Part<1, DAY> + Part<2, DAY> {
    /// Solves the puzzle for that part 1 of the day. 
    fn part1(input: &str) -> impl Display;

    /// Solves the puzzle for that part 2 of the day. 
    fn part2(input: &str) -> impl Display;
}

// Auto implement the `Day` trait for any type that implements the `Part` trait for a day
impl<T, const DAY: u8> Day<DAY> for T
where
    T: Part<1, DAY> + Part<2, DAY>,
{
    fn part1(input: &str) -> impl Display {
        <T as Part<1, DAY>>::solve(input)
    }

    fn part2(input: &str) -> impl Display {
        <T as Part<1, DAY>>::solve(input)
    }
}

// Nicer API
/// Run the solution of a given part for the given day, returning the result as a string. 
///
/// It is mostly used as a nicer way to not need to specify `<Solution as Part<X,
/// Y>>::solve(input).to_string()`.
///
/// See also [`run_day`] to run the entire day. 
pub fn run_day_part<Solutions: Part<PART, DAY>, const DAY: u8, const PART: u8>(
    input: &str,
) -> String {
    Solutions::solve(input).to_string()
}

/// Run the solution of both parts for a given day, returning the result as a string. 
///
/// See also [`run_day_part`] to run an individual part. 
pub fn run_day<Solutions: Day<DAY>, const DAY: u8>(input: &str) -> [String; 2] {
    let part1 = run_day_part::<Solutions, DAY, 1>(input);
    let part2 = run_day_part::<Solutions, DAY, 2>(input);

    [part1, part2]
}

/// Run a day dynamically (with the day itself specified at runtime).
///
/// Assumes there is a full advent of code solution. 
pub fn run<
    Solutions: Day<1>
        + Day<2>
        + Day<3>
        + Day<4>
        + Day<5>
        + Day<6>
        + Day<7>
        + Day<8>
        + Day<9>
        + Day<10>
        + Day<11>
        + Day<12>
        + Day<13>
        + Day<14>
        + Day<15>
        + Day<16>
        + Day<17>
        + Day<18>
        + Day<19>
        + Day<20>
        + Day<21>
        + Day<22>
        + Day<23>
        + Day<24>
        + Day<25>,
>(
    input: &str,
    day: u8,
) -> [String; 2] {
    match day {
        1 => run_day::<Solutions, 1>(input),
        2 => run_day::<Solutions, 2>(input),
        3 => run_day::<Solutions, 3>(input),
        4 => run_day::<Solutions, 4>(input),
        5 => run_day::<Solutions, 5>(input),
        6 => run_day::<Solutions, 6>(input),
        7 => run_day::<Solutions, 7>(input),
        8 => run_day::<Solutions, 8>(input),
        9 => run_day::<Solutions, 9>(input),
        10 => run_day::<Solutions, 10>(input),
        11 => run_day::<Solutions, 11>(input),
        12 => run_day::<Solutions, 12>(input),
        13 => run_day::<Solutions, 13>(input),
        14 => run_day::<Solutions, 14>(input),
        15 => run_day::<Solutions, 15>(input),
        16 => run_day::<Solutions, 16>(input),
        17 => run_day::<Solutions, 17>(input),
        18 => run_day::<Solutions, 18>(input),
        19 => run_day::<Solutions, 19>(input),
        20 => run_day::<Solutions, 20>(input),
        21 => run_day::<Solutions, 21>(input),
        22 => run_day::<Solutions, 22>(input),
        23 => run_day::<Solutions, 23>(input),
        24 => run_day::<Solutions, 24>(input),
        25 => run_day::<Solutions, 25>(input),
        _ => panic!("Day should be between 1 and 25 (inclusive)"),
    }
}
