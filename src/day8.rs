use std::collections::HashMap;

use arrayvec::ArrayVec;
use num_integer::lcm;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> (&str, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();

    let firstline = lines.next().unwrap();

    lines.next();

    let map: HashMap<_, _> = lines
        .map(|l| {
            let [from, rest] = l
                .split(" = ")
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            let rest = &rest[1..rest.len() - 1];

            let to = rest
                .split(", ")
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            (from, to)
        })
        .collect();

    (firstline, map)
}

fn part1(input: &str) {
    let (path, map) = parse_input(input);

    let ans = path
        .chars()
        .cycle()
        .scan("AAA", |pos, rl| {
            let rl = match rl {
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            };

            *pos = map[pos][rl];

            Some(*pos)
        })
        .enumerate()
        .find(|&(_, pos)| matches!(pos, "ZZZ"))
        .unwrap()
        .0
        + 1;

    println!("{ans}");
}

fn part2(input: &str) {
    let (path, map) = parse_input(input);

    let ans = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|&pos| {
            path.chars()
                .cycle()
                .scan(pos, |pos, rl| {
                    let rl = match rl {
                        'L' => 0,
                        'R' => 1,
                        _ => panic!(),
                    };

                    *pos = map[pos][rl];

                    Some(*pos)
                })
                .enumerate()
                .find(|&(_, pos)| pos.ends_with('Z'))
                .unwrap()
                .0
                + 1
        })
        .fold(1, lcm);

    println!("{ans}");
}
