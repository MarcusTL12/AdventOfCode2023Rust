use std::fs::read_to_string;

use arrayvec::ArrayVec;
use home::home_dir;

pub const PARTS: [fn(); 2] = [part1, part2];

type Int = isize;

fn parse_input(input: &str) -> (Vec<Int>, Vec<Vec<[Int; 3]>>) {
    let mut chunks = input.split("\n\n");

    let firstline = chunks.next().unwrap();

    let seeds = firstline
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = chunks
        .map(|c| {
            c.lines()
                .skip(1)
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect::<ArrayVec<_, 3>>()
                        .into_inner()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn map_number(src: Int, map: &[[Int; 3]]) -> Int {
    for &[ds, ss, l] in map {
        let s_end = ss + (l - 1);

        if ss <= src && src <= s_end {
            return ds + src - ss;
        }
    }

    src
}

fn map_number_full(mut src: Int, maps: &[Vec<[Int; 3]>]) -> Int {
    for map in maps {
        src = map_number(src, map);
    }

    src
}

fn find_min_loc<I: Iterator<Item = Int>>(
    seeds: I,
    maps: &[Vec<[Int; 3]>],
) -> Int {
    seeds.map(|seed| map_number_full(seed, maps)).min().unwrap()
}

fn part1() {
    let (seeds, maps) = parse_input(
        &read_to_string(home_dir().unwrap().join("aoc-input/2023/day5/input"))
            .unwrap(),
    );

    let ans = find_min_loc(seeds.iter().cloned(), &maps);

    println!("{ans}");
}

fn _part2_brute_force() {
    let (seeds, maps) = parse_input(
        &read_to_string(home_dir().unwrap().join("aoc-input/2023/day5/input"))
            .unwrap(),
    );

    let ans = seeds
        .iter()
        .array_chunks()
        .map(|[&start, &len]| find_min_loc(start..(start + len), &maps))
        .min()
        .unwrap();

    println!("{ans}");
}

fn rangediff(r1: [Int; 2], r2: [Int; 2]) -> [[Int; 2]; 2] {
    [[r1[0], r1[1].min(r2[0] - 1)], [r1[0].max(r2[1] + 1), r1[1]]]
}

fn rangeintersect(r1: [Int; 2], r2: [Int; 2]) -> [Int; 2] {
    [r1[0].max(r2[0]), r1[1].min(r2[1])]
}

fn map_range(r: [Int; 2], [ds, ss, l]: [Int; 3]) -> [[Int; 2]; 3] {
    let mapped_range = rangeintersect(r, [ss, ss + l - 1]);

    let [remleft, remright] = rangediff(r, mapped_range);

    let ind_range = mapped_range.map(|x| x - ss);

    let dest_range = ind_range.map(|i| ds + i);

    [remleft, remright, dest_range]
}

fn range_empty([a, b]: [Int; 2]) -> bool {
    b < a
}

fn map_min(
    inp_range: [Int; 2],
    map: &[[Int; 3]],
    other_maps: &[Vec<[Int; 3]>],
) -> Int {
    if range_empty(inp_range) {
        return Int::MAX;
    }

    if map.is_empty() {
        return if other_maps.is_empty() {
            inp_range[0]
        } else {
            map_min(inp_range, &other_maps[0], &other_maps[1..])
        };
    }

    let map_r = map[0];
    let map_rest = &map[1..];

    let [rem1, rem2, mapped] = map_range(inp_range, map_r);

    let min1 = if range_empty(mapped) {
        Int::MAX
    } else if other_maps.is_empty() {
        mapped[0]
    } else {
        map_min(mapped, &other_maps[0], &other_maps[1..])
    };

    let min2 = map_min(rem1, map_rest, other_maps);
    let min3 = map_min(rem2, map_rest, other_maps);

    min1.min(min2).min(min3)
}

fn part2() {
    let (seeds, maps) = parse_input(
        &read_to_string(home_dir().unwrap().join("aoc-input/2023/day5/input"))
            .unwrap(),
    );

    let ans = seeds
        .iter()
        .array_chunks()
        .map(|[&start, &len]| {
            map_min([start, (start + len)], &maps[0], &maps[1..])
        })
        .min()
        .unwrap();

    println!("{ans}");
}
