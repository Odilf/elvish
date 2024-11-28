use std::collections::HashMap;

type Entry<'a> = (&'a str, &'a str);
type Map<'a> = HashMap<&'a str, Entry<'a>>;

peg::parser! {
    grammar parser() for str {
        pub rule instructions() -> Vec<u8>
            = i:$(['L' | 'R']+) { i.as_bytes().to_vec() }

        rule node() -> &'input str
            = n:$(['0'..='9' | 'A'..='Z']+) { n }

        rule _  = [' ' | '\n']*

        rule edge() -> (&'input str, Entry<'input>)
            = a:node() _ "=" _ "(" _ b:node() _ ", " _ c:node() _ ")" { (a, (b, c)) }

        rule edges() -> Map<'input>
            = e:edge() ++ _ { e.into_iter().collect() }

        pub rule whole() -> (Vec<u8>, Map<'input>)
            = i:instructions() _ e:edges() _ { (i, e) }
    }
}

fn get_next<'a>(current_node: &str, instructions: &[u8], i: usize, edges: &Map<'a>) -> &'a str {
    let (left, right) = edges.get(current_node).unwrap();
    match instructions[i % instructions.len()] {
        b'L' => left,
        b'R' => right,
        _ => panic!(),
    }
}

#[elvish::solution(day = 8, example = 6)]
fn part1(input: &str) -> i64 {
    let (instructions, edges) = parser::whole(input).unwrap();

    let mut current_node = "AAA";
    for i in 0.. {
        if current_node == "ZZZ" {
            return i as i64;
        }

        current_node = get_next(current_node, &instructions, i, &edges);
    }

    unreachable!()
}

#[elvish::solution(day = 8, example = 6)]
fn part2(input: &str) -> i64 {
    let (instructions, edges) = parser::whole(input).unwrap();

    let ends_with = |node: &str, char: u8| node.as_bytes()[2] == char;

    let starts = edges.keys().filter(|node| ends_with(node, b'A'));

    starts
        .map(|&start| {
            let mut current_node = start;
            for i in 0.. {
                if ends_with(current_node, b'Z') {
                    return i as i64;
                }

                current_node = get_next(current_node, &instructions, i, &edges);
            }

            unreachable!()
        })
        .fold(1, num::integer::lcm)
}

elvish::example!(
    part1: "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    ",

    part2: "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    ",
);
