//! TODO: 

mod decl_macros;

pub use elvish_macros as macros;
pub use elvish_core::*;

pub use color_eyre::eyre::{self, WrapErr as _};
pub use elvish_macros::part;
pub use indoc::indoc;
pub use tracing_subscriber;
pub use dotenvy;

#[derive(clap::Parser)]
struct CliArgs {
    day: Option<u8>,
    part: Option<u8>,
}

/// Copy the output of the solution to the clipboard.
pub fn copy_to_clipboard(input: &str) -> eyre::Result<()> {
    arboard::Clipboard::new()?.set_text(input)?;

    Ok(())
}
