use std::array;

elvish::day!(15);

fn hash(input: &str) -> i64 {
    let mut current = 0;
    for byte in input.bytes() {
        current += byte as i64;
        current *= 17;
        current %= 256;
    }

    current
}

fn part1(input: &str) -> i64 {
    input.trim().split(',').map(|s| hash(s)).sum()
}

fn part2<'a>(input: &'a str) -> i64 {
    let mut boxes: [_; 256] = array::from_fn(|_| Vec::<(&str, i64)>::new());

    for s in input.trim().split(',') {
        None.or_else(|| {
            let mut iter = s.split('=');
            let chars = iter.next().unwrap();
            let hash = hash(chars);

            let value = iter.next()?.parse::<i64>().unwrap();

            let bx = &mut boxes[hash as usize];

            if let Some((_, v)) = bx.iter_mut().find(|(s, _)| s.contains(chars)) {
                *v = value;
            } else {
                boxes[hash as usize].push((chars, value));
            }

            Some(())
        })
        .unwrap_or_else(|| {
            let mut iter = s.split('-');
            let chars = iter.next().unwrap();
            let hash = hash(chars);

            let bx = &mut boxes[hash as usize];
            bx.retain(|(s, _)| !s.contains(chars));
        });
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, lens)| {
            lens.iter()
                .enumerate()
                .map(move |(j, &(_, val))| (i + 1) as i64 * (j + 1) as i64 * val)
        })
        .sum()
}

elvish::examples! {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7" => 1320, 145,
}
