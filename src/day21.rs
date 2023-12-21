use std::collections::{HashMap, VecDeque};

use ndarray::ArrayView2;

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn find_plots(
    grid: &ArrayView2<u8>,
    startpos: [usize; 2],
    steps: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    queue.push_back((startpos, 0));
    seen.insert(startpos, 0);

    while let Some(([x, y], l)) = queue.pop_front() {
        for [dx, dy] in [[1, 0], [0, 1], [-1, 0], [0, -1]] {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            let npos = [nx, ny];

            if grid.get(npos).unwrap_or(&b'#') != &b'#'
                && !seen.contains_key(&npos)
                && l < steps
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

fn part2(_input: &str) {}
