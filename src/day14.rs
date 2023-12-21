use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use ndarray::{
    parallel::prelude::{IntoParallelIterator, ParallelIterator},
    s, ArrayView2, ArrayViewMut1, ArrayViewMut2, Axis,
};

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn fall_up_col(mut col: ArrayViewMut1<u8>) {
    let mut n = 0;

    for i in (0..col.len()).rev() {
        match col[i] {
            b'O' => {
                n += 1;
                col[i] = b'.';
            }
            b'#' => {
                for j in (i + 1)..(i + 1 + n) {
                    col[j] = b'O';
                }
                n = 0;
            }
            _ => {}
        }
    }

    for j in 0..n {
        col[j] = b'O';
    }
}

fn fall_up(mut grid: ArrayViewMut2<u8>) {
    grid.axis_iter_mut(Axis(1))
        .into_par_iter()
        .for_each(fall_up_col);
}

fn calc_load(grid: ArrayView2<u8>) -> usize {
    let (h, _) = grid.dim();

    grid.rows()
        .into_iter()
        .enumerate()
        .map(|(i, r)| r.into_iter().filter(|&&x| x == b'O').count() * (h - i))
        .sum()
}

fn part1(input: &str) {
    let mut grid = parse_grid(input).to_owned();

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

fn part2(input: &str) {
    let mut grid = parse_grid(input).to_owned();

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
