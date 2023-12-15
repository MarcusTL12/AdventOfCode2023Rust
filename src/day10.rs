use arrayvec::ArrayVec;
use ndarray::{s, Array2, ArrayView2};

pub const PARTS: [fn(&str); 2] = [part1, part2];

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn parse_input(input: &str) -> ArrayView2<u8> {
    let b = input.as_bytes();

    let w = b.split(|&x| x == b'\n').next().unwrap().len() + 1;
    let h = b.len() / w;

    ArrayView2::from_shape((h, w), b)
        .unwrap()
        .slice_move(s![0..h, 0..(w - 1)])
}

fn connections(c: u8) -> [[isize; 2]; 2] {
    match c {
        b'-' => [[0, -1], [0, 1]],
        b'|' => [[-1, 0], [1, 0]],
        b'7' => [[0, -1], [1, 0]],
        b'J' => [[0, -1], [-1, 0]],
        b'L' => [[0, 1], [-1, 0]],
        b'F' => [[0, 1], [1, 0]],
        _ => [[0, 0], [0, 0]],
    }
}

fn find_start(grid: &ArrayView2<u8>) -> [usize; 2] {
    let ((y, x), _) = grid.indexed_iter().find(|(_, &x)| x == b'S').unwrap();

    [y, x]
}

fn do_step(
    grid: &ArrayView2<u8>,
    p: [usize; 2],
    d: [isize; 2],
) -> ([usize; 2], [isize; 2]) {
    let [c1, c2] = connections(grid[p]);

    let [dy, dx] = if [-d[0], -d[1]] == c1 { c2 } else { c1 };

    (
        [(p[0] as isize + dy) as usize, (p[1] as isize + dx) as usize],
        [dy, dx],
    )
}

fn part1(input: &str) {
    let grid = parse_input(input);

    let [sy, sx] = find_start(&grid);

    let [(mut p1, mut d1), (mut p2, mut d2)] = DIRS
        .into_iter()
        .filter_map(|[dy, dx]| {
            let p = [(sy as isize + dy) as usize, (sx as isize + dx) as usize];
            grid.get(p).and_then(|&c| {
                connections(c)
                    .contains(&[-dy, -dx])
                    .then_some((p, [dy, dx]))
            })
        })
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    let mut ans = 1;

    while p1 != p2 {
        (p1, d1) = do_step(&grid, p1, d1);
        (p2, d2) = do_step(&grid, p2, d2);
        ans += 1;
    }

    println!("{ans}");
}

fn get_rot_num(d1: [isize; 2], d2: [isize; 2]) -> isize {
    d1[1] * d2[0] - d1[0] * d2[1]
}

fn rot_dir(d: [isize; 2]) -> [isize; 2] {
    [d[1], -d[0]]
}

fn part2(input: &str) {
    let grid = parse_input(input);

    let mut pos = find_start(&grid);
    let [sy, sx] = &pos;

    let start = pos;

    let mut dir = DIRS
        .into_iter()
        .find(|[dy, dx]| {
            let p =
                [(*sy as isize + dy) as usize, (*sx as isize + dx) as usize];
            grid.get(p)
                .map(|&c| connections(c).contains(&[-dy, -dx]))
                .unwrap_or(false)
        })
        .unwrap();

    pos = [
        (pos[0] as isize + dir[0]) as usize,
        (pos[1] as isize + dir[1]) as usize,
    ];

    let mut path = vec![pos];

    let mut totalrot = 0;

    let mut inout = Array2::from_shape_fn(grid.raw_dim(), |_| 0);

    loop {
        let (npos, ndir) = do_step(&grid, pos, dir);

        totalrot += get_rot_num(dir, ndir);

        let mut side_dir = ndir;
        let mut side = 1;
        for _ in 0..3 {
            side_dir = rot_dir(side_dir);

            if side_dir == dir.map(|x| -x) {
                side = 2;
            }

            if let Some(c) = inout.get_mut([
                (pos[0] as isize + side_dir[0]) as usize,
                (pos[1] as isize + side_dir[1]) as usize,
            ]) {
                *c = side;
            }
        }

        pos = npos;
        dir = ndir;
        path.push(pos);

        if pos == start {
            break;
        }
    }

    for &pos in &path {
        inout[pos] = 3;
    }

    let looking_for = if totalrot < 0 { 2 } else { 1 };

    let mut fill_stack = Vec::new();

    for ((y, x), &c) in inout.indexed_iter() {
        if c == looking_for {
            fill_stack.push([y, x]);
        }
    }

    while let Some([y, x]) = fill_stack.pop() {
        for [dy, dx] in DIRS {
            let np = [(y as isize + dy) as usize, (x as isize + dx) as usize];

            if inout[np] == 0 {
                inout[np] = looking_for;
                fill_stack.push(np);
            }
        }
    }

    let ans = inout.iter().filter(|&&x| x == looking_for).count();

    println!("{ans}");
}
