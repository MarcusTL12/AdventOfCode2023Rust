use std::{fs::read_to_string, time::Instant};

use ndarray::{s, ArrayView1, ArrayView2};

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn find_tot_dist(path: &str, expand: usize) -> usize {
    let input = read_to_string(path).unwrap();
    let grid = parse_input(&input);

    let galaxies: Vec<_> = grid
        .indexed_iter()
        .filter_map(|(coord, &c)| (c == b'#').then_some(coord))
        .collect();

    let cumulative_columns: Vec<_> = grid
        .columns()
        .into_iter()
        .scan(0, |n, col| {
            if col.iter().all(|&c| c == b'.') {
                *n += 1;
            }

            Some(*n)
        })
        .collect();

    let cumulative_rows: Vec<_> = grid
        .rows()
        .into_iter()
        .scan(0, |n, row| {
            if row.iter().all(|&c| c == b'.') {
                *n += 1;
            }

            Some(*n)
        })
        .collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &c1)| {
            galaxies.iter().take(i).map(move |&c2| (c1, c2)).map(
                |((y1, x1), (y2, x2))| {
                    let mandist = x1.abs_diff(x2) + y1.abs_diff(y2);

                    let extra_dist = cumulative_columns[x1.max(x2)]
                        - cumulative_columns[x1.min(x2)]
                        + cumulative_rows[y1.max(y2)]
                        - cumulative_rows[y1.min(y2)];

                    mandist + extra_dist * expand
                },
            )
        })
        .sum()
}

fn get_galaxy_coords<'a, I: Iterator<Item = ArrayView1<'a, u8>>>(
    col_iter: I,
    expand: usize,
) -> Vec<usize> {
    col_iter
        .scan(0, |x, col| {
            *x += 1;
            if col.iter().all(|&c| c == b'.') {
                *x += expand;
            }
            Some((*x, col))
        })
        .flat_map(|(x, col)| {
            col.into_iter()
                .filter_map(move |&c| (c == b'#').then_some(x))
        })
        .collect()
}

fn get_sum_dists(coords: &[usize]) -> usize {
    let l = coords.len();

    let (first, last) = coords.split_at(l / 2);

    first
        .iter()
        .zip(last.iter().rev())
        .scan(l + 1, |c, x| {
            *c -= 2;
            Some((*c, x))
        })
        .map(|(c, (&a, &b))| c * a.abs_diff(b))
        .sum()
}

fn find_tot_dist2(path: &str, expand: usize) -> usize {
    let input = read_to_string(path).unwrap();
    let grid = parse_input(&input);

    let galaxy_xs = get_galaxy_coords(grid.columns().into_iter(), expand);
    let galaxy_ys = get_galaxy_coords(grid.rows().into_iter(), expand);

    get_sum_dists(&galaxy_xs) + get_sum_dists(&galaxy_ys)
}

fn part1(path: &str) {
    println!("{}", find_tot_dist(path, 1));
}

fn part2(path: &str) {
    println!("{}", find_tot_dist2(path, 1000000 - 1));
}
