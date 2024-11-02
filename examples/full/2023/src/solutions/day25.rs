use std::collections::{HashMap, HashSet};

elvish::day!(25);

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut neighbors = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split(": ");
        let from = iter.next().unwrap();
        let tos = iter.next().unwrap().split(' ');

        neighbors
            .entry(from)
            .or_insert_with(Vec::new)
            .extend(tos.clone());

        for to in tos {
            neighbors.entry(to).or_insert_with(Vec::new).push(from);
        }
    }

    neighbors
}

fn part1(input: &str) -> i64 {
    let neighbors = parse(input);
    let nodes = neighbors.keys().copied().collect::<Vec<_>>();

    let mut split = nodes.iter().collect::<HashSet<_>>();

    loop {
        let count = |node: &str| {
            neighbors[node]
                .iter()
                .filter(|&neighbor| !split.contains(neighbor))
                .count() as i64
        };

        let counts = split.iter().map(|&&node| (node, count(node)));

        if counts.clone().map(|(_, c)| c).sum::<i64>() == 3 {
            break;
        }

        let (item_to_remove, _) = counts.max_by_key(|&(_, c)| c).unwrap();

        split.remove(&item_to_remove);
    }
    
    let split = split.len() as i64;
    let total = nodes.len() as i64;

    split * (total - split)
}

fn part2(_: &str) -> &'static str {
    "Merry Christmas! 🥂"
}

elvish::examples! {
    part1 { test: "
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    " => 54 } 
}
