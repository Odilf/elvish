//! Macros for elvish. 

#![warn(missing_docs)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod declare;
mod example;
mod solution;

/// A solution of an advent of code problem. 
///
/// You need to pass in `day = X` for the macro to work. If the function is named `part1` or 
/// `part2`, the part gets set accordingly; otherwise you need to specify it as `part = Y`.
///
/// You can also specify the expected result of the example given in the puzzle using `example = Z`
/// or `example = [A, B, C, ...]` if there are multiple. The examples need to be defined somewhere
/// using [`elvish::example!()`](example!()) for them to work. 
///
/// At the end of the day, this macro is mostly to reduce boilerplate but it's easily expandable by
/// hand. 
///
/// # Example usage
///
/// Solution for day 1 of 2023:
///
/// ```rust
/// # struct Solutions;
/// #[elvish::solution(day = 1, example = 142)]
/// fn part1(input: &str) -> u32 {
///     input
///         .lines()
///         .filter(|line| !line.is_empty())
///         .map(|line| {
///             let mut iter = line.chars().filter_map(|c| c.to_digit(10));
/// 
///             let a = iter.next().unwrap();
///             let b = iter.last().unwrap_or(a);
/// 
///             a * 10 + b
///         })
///         .sum()
/// }
/// ```
///
/// which generates:
/// 
/// ```rust
/// # struct Solutions;
/// # const EXAMPLE_PART1: &str = "yo";
/// impl elvish::solution::Part<1, 1> for crate::Solutions {
///     fn solve(input: &str) -> impl std::fmt::Display {
///         part1(input)
///     }
/// }
/// 
/// #[test]
/// fn part1_example() {
///     assert_eq!(part1(EXAMPLE_PART1), 142)
/// }
///
/// fn part1(input: &str) -> u32 {
/// // --snip--
/// }
/// ```
#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    solution::expand(attr, item)
}

/// Defines examples given in advent of code puzzles. The strings in the example are unindented
/// using [`indoc`](https://docs.rs/indoc). 
///
/// There are three cases that make up 90% of examples in advent of code, which this macro
/// addresses. Namely:
///
/// - One example is given for both part 1 and part 2:
///
/// ```rust
/// elvish::example!("
///     YOUR
///     EXAMPLE
///     HERE
/// ");
/// ```
/// - Part 1 and part 2 have each one example:
///
/// ```rust
/// elvish::example!(
///     part1: "
///         YOUR
///         PART 1
///         EXAMPLE
///         HERE
///     ",
///
///     part2: "
///         YOUR
///         PART 2
///         EXAMPLE
///         HERE
///     ",
/// );
/// ```
///
/// - Part 1 and part 2 have more than one example:
///
/// ```rust
/// elvish::example!(
///     part1: "
///         YOUR
///         FIRST PART 1
///         EXAMPLE
///         HERE
///     ",
///
///     part1: "
///         YOUR
///         SECOND PART 1
///         EXAMPLE
///         HERE
///     ",
///
///     part2: "
///         YOUR
///         FIRST PART 2
///         EXAMPLE
///         HERE
///     ",
///
///     part2: "
///         YOUR
///         SECOND PART 2
///         EXAMPLE
///         HERE
///     ",
/// );
/// ```
#[proc_macro]
pub fn example(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as example::Example).expand()
}

/// Declare modules for each day of advent of code.
///
/// Expands to 
/// 
/// ```rust
/// #[cfg(feature="day01")]
/// mod day01;
/// #[cfg(feature="day02")]
/// mod day02;
/// #[cfg(feature="day03")]
/// mod day03;
///
/// // etc...
/// ```
#[proc_macro]
pub fn declare_modules(_input: TokenStream) -> TokenStream {
    declare::modules()
}

/// Declare a function that can run advent of code solutions dynamically based on the
/// aviable (think, solved) days.
///
/// Expands to 
/// 
/// ```rust
/// fn run_day_part(day: u8, part: u8, input: &str) -> eyre::Result<String> {
///     #[cfg(feature = "day01")]
///     if day == 01 {
///         #[cfg(feature = "part1")]
///         if part == 0 {
///             return Ok(elvish::solution::run_day_part::<Solutions, 17u8, 1>(input));
///         }
///         #[cfg(feature = "part2")]
///         if part == 1 {
///             return Ok(elvish::solution::run_day_part::<Solutions, 17u8, 2>(input));
///         }
///     }
///
///     #[cfg(feature = "day02")]
///     if day == 02 {
///         #[cfg(feature = "part1")]
///         if part == 0 {
///             return Ok(elvish::solution::run_day_part::<Solutions, 17u8, 1>(input));
///         }
///         #[cfg(feature = "part2")]
///         if part == 1 {
///             return Ok(elvish::solution::run_day_part::<Solutions, 17u8, 2>(input));
///         }
///     }
/// }
/// 
/// // etc...
/// ```
#[proc_macro]
pub fn declare_run_fn(_input: TokenStream) -> TokenStream {
    declare::run_fn()
}

/// Declares an array of available days, based on feature flags.
///
/// Expands to 
/// 
/// ```rust
/// [
///
///     #[cfg(feature="day01")]
///     1,
///     #[cfg(feature="day02")]
///     2,
///     #[cfg(feature="day03")]
///     3,
///
///     // etc...
/// ]
///
/// ```
#[proc_macro]
pub fn available_days(_input: TokenStream) -> TokenStream {
    declare::available_days()
}
