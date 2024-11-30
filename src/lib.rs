//! # elvish
//! 
//! > Overengineered Advent of Code framework for Rust - not quite Santa's elves. 
//! 
//! elvish is a framework for writing your Advent of Code solutions with the least amount of boilerplate as possible. 
//! 
//! ## Features 
//! 
//! - Declare solutions with simple macro
//! - Fetching and caching of user input
//! - Run solutions as binary
//! - Automatically copy solutions to clipboard
//! - Simple and consice syntax to write out 90% of required tests
//! - See the puzzle description as docs on the annotated function
//! - Conditional compilation to compile a single day

#![warn(missing_docs)]

pub use elvish_core::*;
pub use elvish_macros as macros;

pub use color_eyre::eyre;
pub use elvish_macros::{available_days, example, solution};
pub use indoc::indoc;

/// Convinience for declaring 25-sets with feature flags at once.
pub mod declare {
    pub use elvish_macros::declare_modules as modules;
    pub use elvish_macros::declare_run_fn as run_fn;
}

use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    day: Option<u8>,
    part: Option<u8>,
}

/// Copy the output of the solution to the clipboard.
pub fn copy_to_clipboard(input: &str) -> eyre::Result<()> {
    arboard::Clipboard::new()?.set_text(input)?;

    Ok(())
}

/// Runs elvish. 
///
/// This gives a cli program that can take a day, or auto detect it.
///
/// The two arguments needed can be obtained by using the provided macros
/// [`available_days!()`] and [`declare::run_fn`].
///
/// # Example
///
/// ```no_run
/// // In main.rs
///
/// use color_eyre::eyre;
/// pub struct Solutions;
/// 
/// elvish::declare::run_fn!();
/// 
/// fn main() -> eyre::Result<()> {
///     tracing_subscriber::fmt().init();
///     dotenvy::dotenv()?;
/// 
///     elvish::run::<2023>(&elvish::available_days!(), run_day_part)?;
/// 
///     Ok(())
/// }
/// ```
pub fn run<const YEAR: i16>(
    available_days: &[u8],
    run_day_part: impl Fn(u8, u8, &str) -> eyre::Result<String>,
) -> eyre::Result<()> {
    let args = CliArgs::parse();

    let day = match (available_days, args.day) {
        (&[day], arg) => {
            if let Some(arg) = arg {
                if arg != day {
                    tracing::warn!("Ignoring day argument given (day {arg}), since the only runnable day is {day}")
                }
            }

            day
        }

        (_, Some(arg)) => {
            if !available_days.contains(&arg) {
                eyre::bail!("Day {arg} is not runnable. Available days are: {available_days:?}");
            }

            arg
        }

        (_, None) => eyre::bail!(
            "Please pass a day to run with `--day`. Available days are: {available_days:?}"
        ),
    };

    let session_token = crate::data::get_session_token()?;
    let input = crate::data::get(YEAR, day, &session_token)?.input;

    let run_part = |part| -> eyre::Result<()> {
        let output = run_day_part(day, part, &input)?;
        println!(
            "Solution for day {day} part {part} is: {output}",
            part = part + 1
        );
        copy_to_clipboard(&output)?;
        Ok(())
    };

    match args.part {
        Some(part) => run_part(part - 1)?,
        None => {
            run_part(0)?;
            run_part(1)?;
        }
    };

    Ok(())
}
