use color_eyre::eyre;

/// Data for a day's puzzle.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Day {
    /// The puzzle's input, which is generated uniquely for each user.
    pub input: String,
    pub description_1: Option<String>,
    pub description_2: Option<String>,
}

fn get_env_year() -> eyre::Result<u16> {
    Ok(std::env::var("YEAR")?.parse()?)
}

pub fn get_year() -> eyre::Result<u16> {
    match get_env_year() {
        Ok(year) => Ok(year),
        Err(_) => {
            tracing::warn!("No YEAR environment variable found, using current year");
            let year = jiff::Zoned::now().year();
            Ok(year as u16)
        }
    }
}

pub fn get_session_token() -> eyre::Result<String> {
    Ok(std::env::var("SESSION_TOKEN")?)
}

/// Get the day's [data](Day).
pub fn get(year: u16, day: u8, session_token: &str) -> eyre::Result<Day> {
    read_day(day).or_else(|_| {
        let data = fetch_day(year, day, session_token)?;
        let serialized = ron::to_string(&data)?;

        std::fs::create_dir_all(PARENT_PATH)?;
        std::fs::write(path(day), serialized)?;

        Ok(data)
    })
}

fn fetch_day(year: u16, day: u8, session_token: &str) -> eyre::Result<Day> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let client = reqwest::blocking::Client::new();

    let puzzle_input = client
        .get(&url)
        .header("Cookie", format!("session={session_token}"))
        .send()?
        .error_for_status()?
        .text()?;

    Ok(Day {
        input: puzzle_input,
        description_1: None,
        description_2: None,
    })
}

const PARENT_PATH: &str = ".elvish";
fn path(day: u8) -> impl AsRef<std::path::Path> {
    format!("{PARENT_PATH}/day{:02}.ron", day)
}

fn read_day(day: u8) -> eyre::Result<Day> {
    let day = std::fs::read_to_string(path(day))?;
    let day = ron::from_str(&day)?;

    Ok(day)
}
