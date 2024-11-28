use std::collections::{HashMap, VecDeque};

elvish::day!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pulse {
    Low,
    High,
}

impl From<bool> for Pulse {
    fn from(b: bool) -> Self {
        if b {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjuction(HashMap<&'a str, Pulse>),
    Passthrough,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    from: &'a str,
    to: Vec<&'a str>,
    typ: ModuleType<'a>,
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule module_type() -> ModuleType<'input>
            = "%" { ModuleType::FlipFlop(false) }
            / "&" { ModuleType::Conjuction(HashMap::new()) }
            / "" { ModuleType::Passthrough }

        rule name() -> &'input str
            = n:$(['a'..='z']+) { n }

        pub rule module() -> (&'input str, Module<'input>)
            = typ:module_type() from:name()
              " -> " to:name() ** ", "
              { (from, Module { from, to, typ }) }
    }
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, Module<'_>>,
    HashMap<&str, Vec<&str>>,
    Vec<&str>,
) {
    let mut map = input
        .lines()
        .map(|line| parser::module(line).unwrap())
        .collect::<HashMap<_, _>>();

    let nodes = map.keys().copied().collect::<Vec<_>>();

    let mut inverse = HashMap::new();

    for node in &nodes {
        let module = map.get(node).unwrap();

        for to in &module.to {
            inverse.entry(*to).or_insert_with(Vec::new).push(*node);
        }
    }

    for node in &nodes {
        let module = map.get_mut(node).unwrap();

        if let ModuleType::Conjuction(memory) = &mut module.typ {
            for parent in &inverse[module.from] {
                memory.insert(parent, Pulse::Low);
            }
        }
    }

    (map, inverse, nodes)
}

fn press_button(map: &mut HashMap<&str, Module<'_>>, mut f: impl FnMut(&str, Pulse)) {
    let mut queue = VecDeque::from([("", "broadcaster", Pulse::Low)]);

    while let Some((from, name, pulse)) = queue.pop_front() {
        f(name, pulse);

        let Some(module) = map.get_mut(name) else {
            // Because sometimes outputs have no sends
            continue;
        };

        match &mut module.typ {
            ModuleType::FlipFlop(on) => match pulse {
                Pulse::Low => {
                    *on = !*on;
                    for to in &module.to {
                        queue.push_back((name, to, Pulse::from(*on)));
                    }
                }
                Pulse::High => (),
            },
            ModuleType::Conjuction(memory) => {
                memory.insert(from, pulse);

                let pulse = if memory.values().all(|pulse| *pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for to in &module.to {
                    queue.push_back((name, to, pulse));
                }
            }
            ModuleType::Passthrough => {
                for to in &module.to {
                    queue.push_back((name, to, pulse));
                }
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let (mut map, _inv, _nodes) = parse(input);

    let mut count = [0, 0];

    for _ in 0..1000 {
        press_button(&mut map, |_name, pulse| count[pulse as usize] += 1);
    }

    dbg!(&count);

    count.iter().product()
}

fn part2(input: &str) -> i64 {
    let (mut map, inverse, _nodes) = parse(input);
    const TARGET: &str = "rx";

    let parent = &inverse[TARGET];
    assert_eq!(parent.len(), 1);

    let parent = parent[0];
    assert!(matches!(&map[parent].typ, ModuleType::Conjuction(_)));

    let mut until_parents = inverse[parent]
        .iter()
        .map(|node| (node.to_string(), 0))
        .collect::<HashMap<_, _>>();

    for i in 1.. {
        press_button(&mut map, |name, pulse| {
            if let Some(first_press) = until_parents.get_mut(name) {
                if *first_press == 0 && pulse == Pulse::Low {
                    *first_press = i;
                }
            }
        });

        if until_parents.values().all(|v| *v > 0) {
            return until_parents.values().product();
        }
    }

    panic!("Never turns on (or at least until the i64 limit)");
}

elvish::examples! {
    part1 {
        one: "
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        " => 32_000_000,

        two: "
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        " => 11_687_500,
    }
}
