use std::fs::read_to_string;

use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn get_num_ways(t: u64, d: u64) -> u64 {
    let mut n = 0;
    for ta in 0..t {
        let t_cruise = t - ta;
        let d_cruise = ta * t_cruise;
        if d_cruise > d {
            n += 1;
        }
    }
    n
}

fn part1(path: &str) {
    let input = read_to_string(path).unwrap();

    let [times, dists] = input
        .lines()
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    let ans: u64 = times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| get_num_ways(t, d))
        .product();

    println!("{ans}");
}

fn part2(path: &str) {
    let input = read_to_string(path).unwrap();

    let [time, dist] = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|&c| c.is_ascii_digit())
                .map(|c| ((c as u8) - b'0') as u64)
                .fold(0, |n, x| 10 * n + x)
        })
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    let ans = get_num_ways(time, dist);

    println!("{ans}");
}
