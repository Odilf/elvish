elvish::day!(9);

fn predict(values: &[i64], fold: &impl Fn(&[i64], i64) -> i64) -> i64 {
    if values.iter().all(|v| *v == 0) {
        return 0;
    }

    let deltas = values
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    
    fold(values, predict(&deltas, fold))
}

fn solve(input: &str, fold: &impl Fn(&[i64], i64) -> i64) -> i64 {
    input
        .lines()
        .map(|line| {
            let row = line
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<_>>();

            predict(&row, fold)
        })
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, &|values, delta| values.last().unwrap() + delta)
}

fn part2(input: &str) -> i64 {
    solve(input, &|values, delta| values[0] - delta)
}

elvish::examples! {
    r"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    " => 114, 2,
}
