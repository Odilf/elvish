fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    let mut lines = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).map(|s| s.parse().unwrap()));

    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times.zip(distances)
}

fn parse_joined(input: &str) -> (i64, i64) {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    });

    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    (time, distance)
}

#[elvish::solution(day = 6, example = 288)]
fn part1(input: &str) -> i64 {
    parse(input)
        .map(|(time, distance)| {
            (1..=time)
                .map(|t| {
                    let remaining = time - t;
                    let accelerated = t;
                    accelerated * remaining
                })
                .filter(|&d| d > distance)
                .count() as i64
        })
        .product()
}

#[elvish::solution(day = 6, example = 71503)]
fn part2(input: &str) -> i64 {
    let (time, distance) = parse_joined(input);

    let find = |t| {
        let remaining = time - t;
        let accelerated = t;
        if accelerated * remaining > distance {
            Some(t)
        } else {
            None
        }
    };

    let lower_bound = (1..=time).find_map(find).unwrap();
    let upper_bound = (1..=time).rev().find_map(find).unwrap();

    upper_bound - lower_bound + 1
}

elvish::example!(
    "
        Time:      7  15   30
        Distance:  9  40  200
    "
);
