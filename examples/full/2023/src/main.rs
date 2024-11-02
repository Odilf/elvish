use color_eyre::eyre;

mod solutions;

pub struct Solutions;

// Equivalent to
//
// ```rust
// const YEAR: u16 = 2023;
// ```
elvish::year!(2023);

// Generates the following code:
// ```rust
// #[cfg(feature = "all")]
// fn main() -> eyre::Result<()> {
//     elvish::dotenvy::dotenv()?;
//     let session_token = elvish::data::get_session_token()?;
//     for day in 1..=25 {
//         let day_data = elvish::data::get(YEAR, day, &session_token)?;
//         elvish::solution::run_day::<Solutions, _>(&day_data.input);
//     }
//     Ok(())
// }
// 
// #[cfg(not(feature = "all"))]
// fn main() -> eyre::Result<()> {
//     #[cfg(feature = "day01")]
//     const DAY: u8 = 1;
//     #[cfg(feature = "day02")]
//     const DAY: u8 = 2;
//     // ...and so on
// 
//     elvish::dotenvy::dotenv()?;
// 
//     let session_token = elvish::data::get_session_token()?;
//     let day_data = elvish::data::get(YEAR, DAY, &session_token)?;
// 
//     let (part1, part2) = elvish::solution::run_day::<Solutions, DAY>(&day_data.input);
// 
//     Ok(())
// }
// ```

elvish::main!();

