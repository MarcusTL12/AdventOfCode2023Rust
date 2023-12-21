use ndarray::{Array2, ArrayView2};

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn shift_pos([y, x]: [usize; 2], dir: usize) -> [usize; 2] {
    match dir {
        0 => [y, x + 1],
        1 => [y + 1, x],
        2 => [y, x.wrapping_sub(1)],
        3 => [y.wrapping_sub(1), x],
        _ => panic!(),
    }
}

fn find_energized(
    grid: ArrayView2<u8>,
    energized: &mut Array2<u8>,
    mut pos: [usize; 2],
    dir: usize,
) {
    if energized
        .get(pos)
        .map(|e| e & (1 << dir) != 0)
        .unwrap_or(true)
        && matches!(grid.get(pos), Some(b'.'))
    {
        return;
    }

    while let Some(b'.') = grid.get(pos) {
        energized[pos] |= 1 << dir;
        pos = shift_pos(pos, dir);
    }

    match (grid.get(pos), dir) {
        (Some(b'|'), 1 | 3) | (Some(b'-'), 0 | 2) => {
            energized[pos] |= 1 << dir;
            find_energized(grid, energized, shift_pos(pos, dir), dir);
        }
        (Some(b'|'), _) => {
            energized[pos] |= (1 << 1) | (1 << 3);
            find_energized(grid, energized, shift_pos(pos, 1), 1);
            find_energized(grid, energized, shift_pos(pos, 3), 3);
        }
        (Some(b'-'), _) => {
            energized[pos] |= 1 | (1 << 2);
            find_energized(grid, energized, shift_pos(pos, 0), 0);
            find_energized(grid, energized, shift_pos(pos, 2), 2);
        }
        (Some(b'\\' | b'/'), _) => {
            let ndir = match grid.get(pos) {
                Some(b'\\') => [1, 0, 3, 2],
                Some(b'/') => [3, 2, 1, 0],
                _ => unreachable!(),
            }[dir];

            energized[pos] |= 1 << ndir;
            find_energized(grid, energized, shift_pos(pos, ndir), ndir);
        }
        _ => {}
    }
}

fn part1(input: &str) {
    let grid = parse_grid(input);
    let mut energized = Array2::from_elem(grid.dim(), 0);

    find_energized(grid, &mut energized, [0, 0], 0);

    let ans = energized.iter().filter(|&&c| c != 0).count();

    println!("{ans}");
}

fn part2(input: &str) {
    let grid = parse_grid(input);
    let (h, w) = grid.dim();
    let mut energized = Array2::from_elem(grid.dim(), 0);

    let ans = (0..h)
        .map(|y| {
            find_energized(grid, &mut energized, [y, 0], 0);
            let a = energized.iter().filter(|&&c| c != 0).count();
            for c in energized.iter_mut() {
                *c = 0;
            }
            find_energized(grid, &mut energized, [y, w - 1], 2);
            let b = energized.iter().filter(|&&c| c != 0).count();
            for c in energized.iter_mut() {
                *c = 0;
            }

            a.max(b)
        })
        .max()
        .unwrap();

    let ans = (0..w)
        .map(|x| {
            find_energized(grid, &mut energized, [0, x], 1);
            let a = energized.iter().filter(|&&c| c != 0).count();
            for c in energized.iter_mut() {
                *c = 0;
            }
            find_energized(grid, &mut energized, [h - 1, x], 3);
            let b = energized.iter().filter(|&&c| c != 0).count();
            for c in energized.iter_mut() {
                *c = 0;
            }

            a.max(b)
        })
        .max()
        .unwrap()
        .max(ans);

    println!("{ans}");
}
