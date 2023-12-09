use std::fs::read_to_string;

use arrayvec::ArrayVec;
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

fn part1(path: &str) {
    let input = read_to_string(path).unwrap();

    let grid = parse_input(&input);

    let any_symbol = |x: usize, y: usize, dxys: &[[isize; 2]]| {
        dxys.iter()
            .map(move |&[dx, dy]| {
                [(y as isize + dy) as usize, (x as isize + dx) as usize]
            })
            .map(|xy| grid.get(xy).unwrap_or(&b'.'))
            .any(|&c| c != b'.')
    };

    let (h, w) = grid.dim();

    let mut ans = 0;

    for y in 0..h {
        let mut is_parsing = false;
        let mut found_symbol = false;
        let mut num = 0;

        for x in 0..w {
            let isdigit = grid[[y, x]].is_ascii_digit();

            if !is_parsing && isdigit {
                is_parsing = true;
                found_symbol = any_symbol(
                    x,
                    y,
                    &[[-1, 0], [-1, 1], [-1, -1], [0, -1], [0, 1]],
                );
                num = (grid[[y, x]] - b'0') as u64;
            } else if is_parsing && isdigit {
                found_symbol |= any_symbol(x, y, &[[0, -1], [0, 1]]);
                num = 10 * num + (grid[[y, x]] - b'0') as u64;
            } else if is_parsing {
                found_symbol |= any_symbol(x, y, &[[0, -1], [0, 1], [0, 0]]);
                if found_symbol {
                    ans += num;
                }
                is_parsing = false;
            }
        }
    }

    println!("{ans}");
}

fn part2(path: &str) {
    let input = read_to_string(path).unwrap();

    let grid = parse_input(&input);

    let (h, w) = grid.dim();

    let find_start_of_num = |x: usize, y: usize| {
        (0..x)
            .rev()
            .find(|&x| !grid[[y, x]].is_ascii_digit())
            .map(|x| x + 1)
            .unwrap_or(0)
    };

    let mut ans: u64 = 0;

    for y in 0..h {
        for x in 0..w {
            if grid[[y, x]] == b'*' {
                let mut coords = ArrayVec::<_, 2>::new();
                let mut too_many = false;
                for [dx, dy] in [
                    [1, 0],
                    [-1, 0],
                    [0, 1],
                    [0, -1],
                    [1, 1],
                    [-1, 1],
                    [-1, -1],
                    [1, -1],
                ] {
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;
                    if grid[[ny, nx]].is_ascii_digit() {
                        let nx = find_start_of_num(nx, ny);

                        if !coords.contains(&[nx, ny])
                            && coords.try_push([nx, ny]).is_err()
                        {
                            too_many = true;
                            break;
                        }
                    }
                }

                if coords.is_full() && !too_many {
                    ans += coords
                        .into_iter()
                        .map(|[mut x, y]| {
                            let mut n = 0;
                            while grid
                                .get([y, x])
                                .unwrap_or(&b'.')
                                .is_ascii_digit()
                            {
                                n = 10 * n + (grid[[y, x]] - b'0') as u64;
                                x += 1;
                            }
                            n
                        })
                        .product::<u64>();
                }
            }
        }
    }

    println!("{ans}");
}
