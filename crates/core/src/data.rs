use color_eyre::eyre;
use reqwest::blocking::Client;

/// Data for a day's puzzle.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Day {
    /// The puzzle's input, which is generated uniquely for each user.
    pub input: String,
    pub description_1: String,
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
        tracing::warn!("Day data not found in `.elvish`, fetching day...");
        let data = fetch_day(year, day, session_token)?;
        let serialized = ron::to_string(&data)?;

        std::fs::create_dir_all(PARENT_PATH)?;
        std::fs::write(path(day), serialized)?;

        Ok(data)
    })
}

pub fn fetch_day(year: u16, day: u8, session_token: &str) -> eyre::Result<Day> {
    let client = reqwest::blocking::Client::new();
    let (desc1, desc2) = fetch_desc(&client, year, day, session_token)?;

    Ok(Day {
        input: fetch_day_input(&client, year, day, session_token)?,
        description_1: desc1,
        description_2: desc2,
    })
}

fn fetch_aoc(client: &Client, url: &str, session_token: &str) -> eyre::Result<String> {
    let response = client
        .get(url)
        .header("Cookie", format!("session={session_token}"))
        .send()?
        .error_for_status()?
        .text()?;

    Ok(response)
}

fn fetch_day_input(
    client: &Client,
    year: u16,
    day: u8,
    session_token: &str,
) -> eyre::Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    fetch_aoc(client, &url, session_token)
}

fn fetch_desc(client: &Client, year: u16, day: u8, session_token: &str) -> eyre::Result<(String, Option<String>)> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");

    let html = fetch_aoc(client, &url, session_token)?;
    let dom = tl::parse(&html, tl::ParserOptions::default())?;
    let parser = dom.parser();
    let elements = dom.query_selector(".day-desc").expect("There should be at least one element with `day-desc` in AOC pages.");

    let mut descriptions = elements.map(|element| {
        let inner_html = element.get(parser).unwrap().inner_html(parser);
        let markdown = mdka::from_html(&inner_html);

        markdown
    });

    let desc1 = descriptions.next().expect("AOC should have at least one day description marked with a `day-desc` class.");
    let desc2 = descriptions.next();

    Ok((desc1, desc2))
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
