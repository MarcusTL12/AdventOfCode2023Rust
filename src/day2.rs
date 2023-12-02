use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use home::home_dir;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let reg = Regex::new(r"(\d+) (\w+)").unwrap();

    let ans: usize = BufReader::new(
        File::open(home_dir().unwrap().join("aoc-input/2023/day2/input"))
            .unwrap(),
    )
    .lines()
    .map(|l| l.unwrap())
    .enumerate()
    .filter_map(|(id, l)| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for c in reg.captures_iter(&l) {
            let n: usize = c[1].parse().unwrap();

            let p = match &c[2] {
                "red" => &mut r,
                "green" => &mut g,
                "blue" => &mut b,
                _ => panic!(),
            };

            *p = n.max(*p);
        }

        (r <= 12 && g <= 13 && b <= 14).then_some(id + 1)
    })
    .sum();

    println!("{}", ans);
}

fn part2() {
    let reg = Regex::new(r"(\d+) (\w+)").unwrap();

    let ans: usize = BufReader::new(
        File::open(home_dir().unwrap().join("aoc-input/2023/day2/input"))
            .unwrap(),
    )
    .lines()
    .map(|l| l.unwrap())
    .map(|l| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for c in reg.captures_iter(&l) {
            let n: usize = c[1].parse().unwrap();

            let p = match &c[2] {
                "red" => &mut r,
                "green" => &mut g,
                "blue" => &mut b,
                _ => panic!(),
            };

            *p = n.max(*p);
        }

        r * b * g
    })
    .sum();

    println!("{}", ans);
}
