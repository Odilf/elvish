//! Getting data from advent of code, and data required to get data from advent of code (year and
//! session token).

use color_eyre::eyre;
use jiff::civil::Time;
use reqwest::blocking::Client;

/// Data for a day's puzzle.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Day {
    /// The puzzle's input, which is generated uniquely for each user.
    pub input: String,

    /// The description for part one of the puzzle
    pub description_1: String,

    /// Description for part two of the puzzle. It is `None` part 1 hasn't completed yet.
    pub description_2: Option<String>,
}

/// Gets the year from the environment.
pub fn get_env_year() -> eyre::Result<i16> {
    Ok(std::env::var("YEAR")?.parse()?)
}

/// Gets the year from the environment, or returns the current year
pub fn get_year() -> i16 {
    get_env_year().unwrap_or_else(|_| {
        tracing::warn!("No YEAR environment variable found, using current year");
        jiff::Zoned::now().year()
    })
}

/// Gets the session token from the environment
pub fn get_session_token() -> eyre::Result<String> {
    Ok(std::env::var("SESSION_TOKEN")?)
}

/// Checks whether the specified is accessible.
///
/// Assumes system time is correct. 
pub fn is_day_accessible(year: i16, day: u8) -> bool {
    let tz = jiff::tz::offset(-5).to_time_zone();
    let now = jiff::Timestamp::now().to_zoned(tz.clone());

    // If it's a previous year, it's always automatically valid.
    // Course check of the same thing as below.
    if now.year() > year {
        return true;
    }

    let unlocks = jiff::Zoned::now()
        .with()
        .year(year)
        .day(day as i8)
        .month(12)
        .time(Time::midnight())
        .build()
        .expect("Date should be valid")
        .with_time_zone(tz);

    if now > unlocks {
        return true;
    } else {
        return false;
    }
}

/// Get the day's [data](Day).
pub fn get(year: i16, day: u8, session_token: &str) -> eyre::Result<Day> {
    if !is_day_accessible(year, day) {
        let msg = format!("Day {day} is not accessible yet!");
        tracing::warn!(msg);
        eprintln!("{msg}");
        eyre::bail!(msg);
    }

    read_day(day).or_else(|_| {
        tracing::warn!("Day data not found in `.elvish`, fetching day...");
        eprintln!("Day data not found in `.elvish`, fetching day...");
        let data = fetch_day(year, day, session_token)?;
        let serialized = ron::to_string(&data)?;

        std::fs::create_dir_all(PARENT_PATH)?;
        std::fs::write(path(day), serialized)?;

        Ok(data)
    })
}

/// Fetches the data for a day from the advent of code website. Not cached.
pub fn fetch_day(year: i16, day: u8, session_token: &str) -> eyre::Result<Day> {
    let client = reqwest::blocking::Client::new();
    let (desc1, desc2) = fetch_desc(&client, year, day, session_token)?;

    Ok(Day {
        input: fetch_day_input(&client, year, day, session_token)?,
        description_1: desc1,
        description_2: desc2,
    })
}

/// Fetches some url formatting the cookies to include the session token in order to be valid for
/// advent of code
fn fetch_aoc(client: &Client, url: &str, session_token: &str) -> eyre::Result<String> {
    let response = client
        .get(url)
        .header("Cookie", format!("session={session_token}"))
        .send()?
        .error_for_status()?
        .text()?;

    Ok(response)
}

/// Fetches the input for a day's puzzle
fn fetch_day_input(
    client: &Client,
    year: i16,
    day: u8,
    session_token: &str,
) -> eyre::Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    fetch_aoc(client, &url, session_token)
}

/// Fetches the descriptions for a day's puzzle. Returns a tuple of part 1's description and
/// optionally part 2's description
fn fetch_desc(
    client: &Client,
    year: i16,
    day: u8,
    session_token: &str,
) -> eyre::Result<(String, Option<String>)> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");

    let html = fetch_aoc(client, &url, session_token)?;
    let dom = tl::parse(&html, tl::ParserOptions::default())?;
    let parser = dom.parser();
    let elements = dom
        .query_selector(".day-desc")
        .expect("There should be at least one element with `day-desc` in AOC pages.");

    let mut descriptions = elements.map(|element| {
        let inner_html = element.get(parser).unwrap().inner_html(parser);
        let markdown = mdka::from_html(&inner_html);

        markdown
    });

    let desc1 = descriptions
        .next()
        .expect("AOC should have at least one day description marked with a `day-desc` class.");
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
