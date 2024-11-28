pub use elvish_core::*;
pub use elvish_macros as macros;

pub use color_eyre::eyre::{self, WrapErr as _};
pub use elvish_macros::{example, solution, available_days};
pub use indoc::indoc;

pub mod declare {
    pub use elvish_macros::declare_run_fn as run_fns;
    pub use elvish_macros::declare_modules as modules;
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

pub fn run<const YEAR: u16>(
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
