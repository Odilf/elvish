#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    Joker,
    Number(i32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    fn new(char: char, j_is_joker: bool) -> Self {
        if let Some(digit) = char.to_digit(10) {
            Value::Number(digit as i32)
        } else {
            match char {
                'T' => Self::Number(10),
                'J' if !j_is_joker => Self::Jack,
                'J' if j_is_joker => Self::Joker,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                other => panic!("Invalid char: {other}"),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rank {
    bundles: [i32; 5],
}

impl Rank {
    fn new(values: [Value; 5]) -> Self {
        let mut bundles = [0; 5];
        let mut seen = [false; 5];
        let mut jokers = 0;

        for (i, value) in values.iter().enumerate() {
            if *value == Value::Joker {
                jokers += 1;
                continue;
            }

            if seen[i] {
                continue;
            }

            let mut count = 1;

            for (j, other) in values.iter().enumerate().skip(i + 1) {
                if value == other {
                    seen[j] = true;
                    count += 1;
                }
            }

            bundles[i] = count;
        }

        bundles.sort_by_key(|&b| std::cmp::Reverse(b));

        bundles[0] += jokers;

        assert_eq!(bundles.iter().sum::<i32>(), 5);

        Rank { bundles }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bid {
    rank: Rank,
    hand: [Value; 5],
    amount: i64,
}

fn consume<T>(mut iter: impl Iterator<Item = T>) -> [T; 5] {
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn solve(input: &str, j_is_joker: bool) -> i64 {
    let mut bids = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let chars = parts.next().unwrap().chars();

            let hand = consume(chars.map(|c| Value::new(c, j_is_joker)));
            let rank = Rank::new(hand);
            let amount = parts.next().unwrap().parse().unwrap();
            Bid { rank, hand, amount }
        })
        .collect::<Vec<_>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(i, bid)| (i as i64 + 1) * bid.amount)
        .sum()
}

#[elvish::solution(day = 7, example = 6440)]
fn part1(input: &str) -> i64 {
    solve(input, false)
}

#[elvish::solution(day = 7, example = 5905)]
fn part2(input: &str) -> i64 {
    solve(input, true)
}

elvish::example!(
    r"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "
);
