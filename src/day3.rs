use std::fs::read_to_string;

use home::home_dir;
use ndarray::{s, ArrayView2};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn part1() {
    let input =
        read_to_string(home_dir().unwrap().join("aoc-input/2023/day3/ex1"))
            .unwrap();

    let _grid = parse_input(&input);

    // let any_symbol = |x: usize, y: usize, dxys: &[[isize; 2]]| {
    //     dxys.iter()
    //         .map(move |&[dx, dy]| {
    //             [(x as isize + dx) as usize, (y as isize + dy) as usize]
    //         })
    //         .map(|xy| grid.get(xy).unwrap_or(&b'.'))
    //         .any(|&c| c == b'.')
    // };

    // let (h, w) = grid.dim();

    // for y in 0..h {
    //     let mut is_parsing = false;
    //     let mut found_symbol = false;
    //     let mut num = 0;

    //     for x in 0..w {
    //         let isdigit = grid[[y, x]].is_ascii_digit();

    //         if !is_parsing && isdigit {
    //         } else if is_parsing && isdigit {
    //         } else if is_parsing {
    //         }
    //     }
    // }
}

fn part2() {}
