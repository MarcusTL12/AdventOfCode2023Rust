use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(&str); 2] = [part1, part2];

type Int = i64;

fn diff_first(xs: &mut [Int]) -> &mut [Int] {
    let l = xs.len();

    for i in 0..l - 1 {
        xs[i] = xs[i + 1] - xs[i];
    }

    &mut xs[0..l - 1]
}

fn extrapolate(xs: &mut [Int]) -> Int {
    if xs.is_empty() {
        return 0;
    }

    let &lastnum = xs.last().unwrap();

    let dx = diff_first(xs);

    if dx.iter().all(|&x| x == 0) {
        lastnum
    } else {
        lastnum + extrapolate(dx)
    }
}

fn part1(path: &str) {
    let mut buf = Vec::new();

    let ans: Int = BufReader::new(File::open(path).unwrap())
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

fn part2(path: &str) {
    let mut buf = Vec::new();

    let ans: Int = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            buf.clear();
            buf.extend(
                l.split_ascii_whitespace()
                    .map(|x| x.parse::<Int>().unwrap()),
            );
            buf.reverse();
            extrapolate(&mut buf)
        })
        .sum();

    println!("{ans:?}");
}
