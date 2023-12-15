use ndarray::{s, Array2, ArrayView2};

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn has_mirror_at(grid: ArrayView2<u8>, mut i: usize) -> bool {
    let (h, _) = grid.dim();

    let mut j = i - 1;
    while i < h && j < h {
        if grid.row(i) != grid.row(j) {
            return false;
        }

        i += 1;
        j = j.wrapping_sub(1);
    }

    true
}

fn find_mirror_plane(grid: ArrayView2<u8>) -> Option<usize> {
    (1..grid.dim().0).find(|&i| has_mirror_at(grid, i))
}

fn part1(input: &str) {
    let ans: usize = input
        .split_inclusive("\n\n")
        .map(parse_input)
        .map(|grid| {
            let a = find_mirror_plane(grid);
            let b = find_mirror_plane(grid.reversed_axes());

            match (a, b) {
                (Some(a), None) => 100 * a,
                (None, Some(b)) => b,
                _ => panic!(),
            }
        })
        .sum();

    println!("{ans}");
}

fn parse_input_mut(input: &str) -> Array2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    let b = b[0..w * h].to_vec();

    Array2::from_shape_vec((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn find_mirror_plane_exclude(
    grid: ArrayView2<u8>,
    not_here: Option<usize>,
) -> Option<usize> {
    (1..grid.dim().0)
        .filter(|&x| Some(x) != not_here)
        .find(|&i| has_mirror_at(grid, i))
}

fn look_for_smudge(mut grid: Array2<u8>) -> usize {
    let a = find_mirror_plane(grid.view());
    let b = find_mirror_plane(grid.view().reversed_axes());

    let (h, w) = grid.dim();

    for i in 0..h {
        for j in 0..w {
            grid[[i, j]] = match grid[[i, j]] {
                b'#' => b'.',
                b'.' => b'#',
                _ => panic!(),
            };

            let x = find_mirror_plane_exclude(grid.view(), a);
            let y = find_mirror_plane_exclude(grid.view().reversed_axes(), b);

            match (x, y) {
                (Some(a), None) => return 100 * a,
                (None, Some(b)) => return b,
                _ => {}
            }

            grid[[i, j]] = match grid[[i, j]] {
                b'#' => b'.',
                b'.' => b'#',
                _ => panic!(),
            };
        }
    }

    panic!()
}

fn part2(input: &str) {
    let ans: usize = input
        .split_inclusive("\n\n")
        .map(parse_input_mut)
        .map(look_for_smudge)
        .sum();

    println!("{ans}");
}
