use color_eyre::eyre;

mod solutions;

pub struct Solutions;

elvish::declare::run_fn!();

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    elvish::run::<2023>(&elvish::available_days!(), run_day_part)?;

    Ok(())
}
