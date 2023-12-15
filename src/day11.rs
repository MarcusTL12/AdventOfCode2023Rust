use ndarray::{s, ArrayView2};

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn get_galaxy_coords<
    'a,
    I2: IntoIterator<Item = &'a u8> + Copy,
    I: Iterator<Item = I2>,
>(
    col_iter: I,
    expand: usize,
) -> Vec<usize> {
    col_iter
        .scan(0, |x, col| {
            *x += 1;
            if col.into_iter().all(|&c| c == b'.') {
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

fn find_tot_dist(input: &str, expand: usize) -> usize {
    let grid = parse_input(input);

    let galaxy_xs = get_galaxy_coords(grid.columns().into_iter(), expand);
    let galaxy_ys = get_galaxy_coords(grid.rows().into_iter(), expand);

    get_sum_dists(&galaxy_xs) + get_sum_dists(&galaxy_ys)
}

fn part1(input: &str) {
    println!("{}", find_tot_dist(input, 1));
}

fn part2(input: &str) {
    println!("{}", find_tot_dist(input, 1000000 - 1));
}
