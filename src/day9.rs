use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use home::home_dir;

pub const PARTS: [fn(); 2] = [part1, part2];

type Int = i64;

fn diff_first(xs: &mut [Int]) -> &mut [Int] {
    let l = xs.len();

    for i in 0..l - 1 {
        xs[i] = xs[i + 1] - xs[i];
    }

    &mut xs[0..l - 1]
}

fn extrapolate(xs: &mut [Int]) -> Int {
    let &lastnum = xs.last().unwrap();

    let dx = diff_first(xs);

    if dx.iter().all(|&x| x == 0) {
        lastnum
    } else {
        lastnum + extrapolate(dx)
    }
}

fn part1() {
    let mut buf = Vec::new();

    let ans: Int = BufReader::new(
        File::open(home_dir().unwrap().join("aoc-input/2023/day9/input"))
            .unwrap(),
    )
    .lines()
    .map(|l| l.unwrap())
    .map(|l| {
        buf.clear();
        buf.extend(
            l.split_ascii_whitespace()
                .map(|x| x.parse::<Int>().unwrap()),
        );
        extrapolate(&mut buf)
    })
    .sum();

    println!("{ans:?}");
}

fn diff_last(xs: &mut [Int]) -> &mut [Int] {
    let l = xs.len();

    for i in (1..l).rev() {
        xs[i] -= xs[i - 1];
    }

    &mut xs[1..]
}

fn extrapolate_back(xs: &mut [Int]) -> Int {
    let &firstnum = xs.first().unwrap();

    let dx = diff_last(xs);

    if dx.iter().all(|&x| x == 0) {
        firstnum
    } else {
        firstnum - extrapolate_back(dx)
    }
}

fn part2() {
    let mut buf = Vec::new();

    let ans: Int = BufReader::new(
        File::open(home_dir().unwrap().join("aoc-input/2023/day9/input"))
            .unwrap(),
    )
    .lines()
    .map(|l| l.unwrap())
    .map(|l| {
        buf.clear();
        buf.extend(
            l.split_ascii_whitespace()
                .map(|x| x.parse::<Int>().unwrap()),
        );
        extrapolate_back(&mut buf)
    })
    .sum();

    println!("{ans:?}");
}
