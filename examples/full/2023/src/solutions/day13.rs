elvish::day!(13);

fn parse(input: &str) -> impl Iterator<Item = (Vec<Vec<bool>>, Vec<Vec<bool>>)> + '_ {
    input.split("\n\n").map(|block| {
        let rows: Vec<Vec<bool>> = block
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let columns: Vec<Vec<bool>> = (0..rows[0].len())
            .map(|x| (0..rows.len()).map(|y| rows[y][x]).collect())
            .collect();

        (rows, columns)
    })
}

fn check_reflections(rows: &[Vec<bool>], index: usize) -> bool {
    let mut i = 0;
    loop {
        if index < i + 1 || index + i >= rows.len() {
            return true;
        }

        if rows[(index - i - 1) as usize] != rows[(index + i) as usize] {
            return false;
        }

        i += 1;
    }
}

/// Returns index n. This means that if there's a reflection in n, then c[n] == c[n - 1], c[n + 1] == c[n - 2], ..., c[n + k] == c[n - k - 1].
fn find_reflections(rows: &[Vec<bool>], ignore: Option<usize>) -> Option<usize> {
    if rows.len() <= 1 {
        return None;
    }

    let half = rows.len() / 2;

    let check = |index: usize| {
        if Some(index) != ignore && check_reflections(rows, index) {
            Some(index)
        } else {
            None
        }
    };

    find_reflections(&rows[..half], ignore)
        .and_then(|first| check(first))
        .or_else(|| {
            find_reflections(&rows[half..], ignore.map(|i| i - half))
                .and_then(|second| check(half + second))
        })
        .or_else(|| check(half))
}

fn part1(input: &str) -> usize {
    parse(input)
        .map(|(rows, columns)| {
            find_reflections(&columns, None)
                .or_else(|| find_reflections(&rows, None).map(|v| v * 100))
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .enumerate()
        .map(|(i, (mut rows, mut columns))| {
            let row_len = rows.len();
            let col_len = columns.len();

            let original_row = find_reflections(&rows, None);
            let original_col = find_reflections(&columns, None);

            (0..row_len).find_map(|y| {
                (0..col_len).find_map(|x| {
                    rows[y][x] = !rows[y][x];
                    columns[x][y] = !columns[x][y];

                    let refl_row = find_reflections(&rows, original_row).map(|v| v * 100);
                    let refl_col = find_reflections(&columns, original_col);

                    rows[y][x] = !rows[y][x];
                    columns[x][y] = !columns[x][y];

                    refl_row.or(refl_col)
                })
            }).unwrap()
        })
        .sum()
}

elvish::examples! {
    "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    " => 405, 400,
}
