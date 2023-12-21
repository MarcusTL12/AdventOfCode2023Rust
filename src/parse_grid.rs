use ndarray::{s, ArrayView2};

pub fn parse_grid(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}
