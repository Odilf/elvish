# elvish

> Overengineered Advent of Code framework for Rust - not quite Santa's elves. 

elvish is a framework for writing your Advent of Code solutions with the least amount of boilerplate as possible. 

## Features 

- Declare solutions with simple macro
- Fetching and caching of user input
- Run solutions as binary
- Automatically copy solutions to clipboard
- Simple and consice syntax to write out 90% of required tests
- See the puzzle description as docs on the annotated function
- Conditional compilation to compile a single day

## Quick example 

This is my solution for day 1 of 2023 using elvish:

```rust
struct Solutions;

#[elvish::solution(day = 1, example = 142)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));

            let a = iter.next().unwrap();
            let b = iter.last().unwrap_or(a);

            a * 10 + b
        })
        .sum()
}

#[elvish::solution(day = 1, example = 281)]
fn part2(input: &str) -> u32 {
    let parse_slice = |slice: &str| {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let digit = slice.chars().next().unwrap().to_digit(10);
        let named = || {
            digits
                .iter()
                .enumerate()
                .find(|(_, &digit)| slice.starts_with(digit))
                .map(|(i, _)| i as u32 + 1)
        };

        digit.or_else(named)
    };

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = (0..line.len()).filter_map(|i| parse_slice(&line[i..]));

            let a = iter.next().unwrap();
            let b = iter.last().unwrap_or(a);

            a * 10 + b
        })
        .sum()
}

elvish::example!(
    part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ", 

    part2: "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "
);

```

# Usage

## Installing

Clone the template repo:

```bash
git clone https://github.com/odilf/elvish-template
```

If you want to use a remote, change the url:

```bash
git remote origin set-url your-repo-url
```

Then, edit the `.env` by setting your session token and the year. 

<details>
<summary> detailed instructions </summary>

To use elvish, first add it as a dependency

```bash
cargo add elvish
```

Then, declare a `Solutions` struct in the root of the crate:


```rust
// in main.rs
struct Solutions;
```

And add a main function:

```rust
// in main.rs
struct Solutions;

elvish::declare::run_fn!();

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    elvish::run::<2023>(&elvish::available_days!(), run_day_part)?;

    Ok(())
}
```

For the data fetching to work you need to add your session token and year in a `.env` file (make sure to `.gitignore` it). 

```bash
SESSION_TOKEN=something
YEAR=202X
```

Finally, you need to add some cargo features to conditionally compile each day:

```toml
[features]
# You can add your own features here

# Detected by `elvish`
today = []
generate-docs = []

part1 = []
part2 = []
both = ["part1", "part2"]

1 = ["part1"]
2 = ["part2"]
b = ["both"]

day01 = []
day02 = []
day03 = []
day04 = []
day05 = []
day06 = []
day07 = []
day08 = []
day09 = []
day10 = []
day11 = []
day12 = []
day13 = []
day14 = []
day15 = []
day16 = []
day17 = []
day18 = []
day19 = []
day20 = []
day21 = []
day22 = []
day23 = []
day24 = []
day25 = []

d01 = ["day01"]
d02 = ["day02"]
d03 = ["day03"]
d04 = ["day04"]
d05 = ["day05"]
d06 = ["day06"]
d07 = ["day07"]
d08 = ["day08"]
d09 = ["day09"]
d10 = ["day10"]
d11 = ["day11"]
d12 = ["day12"]
d13 = ["day13"]
d14 = ["day14"]
d15 = ["day15"]
d16 = ["day16"]
d17 = ["day17"]
d18 = ["day18"]
d19 = ["day19"]
d20 = ["day20"]
d21 = ["day21"]
d22 = ["day22"]
d23 = ["day23"]
d24 = ["day24"]
d25 = ["day25"]

all = [
	"day01",
	"day02",
	"day03",
	"day04",
	"day05",
	"day06",
	"day07",
	"day08",
	"day09",
	"day10",
	"day11",
	"day12",
	"day13",
	"day14",
	"day15",
	"day16",
	"day17",
	"day18",
	"day19",
	"day20",
	"day21",
	"day22",
	"day23",
	"day24",
	"day25",
]
```
</details>

## Running

By default it compiles a binary that includes all days and runs it. To specify a day, you can use feature flags:

```bash
cargo run --no-default-features --features "day01 part1" # or part2
```

or, to run both parts,

```bash
cargo run --no-default-features --features "day01 both"
```

and, as a shorthand

```bash
cargo run --no-default-features --features "d01 b" # p1/p2 for part1/2
```

## Test examples from prompts

elvish provides convinient macros to declare example inputs for 90% of cases:

- One example per part
```rust
#[elvish::solution(day = 1, example = 142)]
fn part1(input: &str) -> i32 {
    // --snip--
}

#[elvish::solution(day = 1, example = 281)]
fn part2(input: &str) -> i32 {
    // --snip--
}

elvish::example!(
    part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",

    part2: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",
);
```

- Same example for both parts
```rust
elvish::example!("
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
");
```

- More than one example for a part
```rust
elvish::example!(
    part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",

    part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",

    part2: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",
);
```


# Other

## Future roadmap

- [ ] Warn when day shouldn't be available yet
