use std::collections::{HashMap, VecDeque};

use ndarray::ArrayView2;
use polynomial::Polynomial;

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn find_plots(
    grid: &ArrayView2<u8>,
    startpos: [usize; 2],
    steps: usize,
) -> usize {
    let (w, h) = grid.dim();

    let startpos = startpos.map(|q| q as isize);

    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    queue.push_back((startpos, 0));
    seen.insert(startpos, 0);

    while let Some(([x, y], l)) = queue.pop_front() {
        for [dx, dy] in [[1, 0], [0, 1], [-1, 0], [0, -1]] {
            let nx = x + dx;
            let ny = y + dy;

            let npos = [nx, ny];
            let npos_mod = [
                nx.rem_euclid(w as isize) as usize,
                ny.rem_euclid(h as isize) as usize,
            ];

            if grid[npos_mod] != b'#' && !seen.contains_key(&npos) && l < steps
            {
                queue.push_back((npos, l + 1));
                seen.insert(npos, l + 1);
            }
        }
    }

    seen.values().filter(|&l| (l % 2) == (steps % 2)).count()
}

fn part1(input: &str) {
    let grid = parse_grid(input);

    let (w, _) = grid.dim();

    let ans = find_plots(&grid, [w / 2, w / 2], 64);

    println!("{ans}");
}

fn part2(input: &str) {
    let grid = parse_grid(input);

    let (w, _) = grid.dim();

    let n_steps_total = 26501365;

    let xs = [0, 1, 2];
    let ys = xs.map(|nw| {
        find_plots(&grid, [w / 2, w / 2], n_steps_total % w + w * (nw as usize))
            as i64
    });

    let poly = Polynomial::lagrange(&xs, &ys).unwrap();

    let ans = poly.eval((n_steps_total / w) as i64);

    println!("{ans}");
}
