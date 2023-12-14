use std::{
    collections::HashMap,
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
};

use ndarray::{s, Array2, ArrayView2, ArrayViewMut2};

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input_mut(input: &str) -> Array2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    let b = b[0..w * h].to_vec();

    Array2::from_shape_vec((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn fall_up(mut grid: ArrayViewMut2<u8>) {
    let (h, w) = grid.dim();

    let mut done = false;

    while !done {
        done = true;

        for y in 1..h {
            for x in 0..w {
                if grid[[y, x]] == b'O' && grid[[y - 1, x]] == b'.' {
                    done = false;
                    grid[[y, x]] = b'.';
                    grid[[y - 1, x]] = b'O';
                }
            }
        }
    }
}

fn calc_load(grid: ArrayView2<u8>) -> usize {
    let (h, _) = grid.dim();

    grid.rows()
        .into_iter()
        .enumerate()
        .map(|(i, r)| r.into_iter().filter(|&&x| x == b'O').count() * (h - i))
        .sum()
}

fn part1(path: &str) {
    let input = read_to_string(path).unwrap();

    let mut grid = parse_input_mut(&input);

    fall_up(grid.view_mut());

    let ans = calc_load(grid.view());

    println!("{ans}");
}

fn do_cycle(mut grid: ArrayViewMut2<u8>) {
    fall_up(grid.view_mut());
    fall_up(grid.view_mut().reversed_axes());
    fall_up(grid.slice_mut(s![..;-1, ..]));
    fall_up(grid.slice_mut(s![.., ..;-1]).reversed_axes());
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part2(path: &str) {
    let input = read_to_string(path).unwrap();

    let mut grid = parse_input_mut(&input);

    let mut seen = HashMap::new();

    let mut i = 0;

    let (loop_start, loop_end) = loop {
        do_cycle(grid.view_mut());
        let hash = calculate_hash(&grid);
        if let Some(j) = seen.get(&hash) {
            break (j, i);
        } else {
            seen.insert(hash, i);
        }

        i += 1;
    };

    let loop_len = loop_end - loop_start;
    let rem_cycle = (1_000_000_000 - loop_start) % loop_len;

    for _ in 1..rem_cycle {
        do_cycle(grid.view_mut());
    }

    let ans = calc_load(grid.view());

    println!("{ans}");
}
