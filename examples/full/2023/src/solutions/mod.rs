elvish::declare::modules!();

// Generates:
//
// ```rust
// #[cfg(or(feature = "day01-1", feature = "day01-2")]
// mod day01;
// #[cfg(or(feature = "day02-1", feature = "day02-2")]
// mod day02;
// #[cfg(or(feature = "day03-1", feature = "day03-2")]
// mod day03;
// // ...
// ```
//
// ...and so on, up to day 25.
