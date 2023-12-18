use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn find_area<I: Iterator<Item = [i64; 2]>>(dirs: I) -> i64 {
    ([[0, 0]].iter().cloned())
        .chain(dirs)
        .scan([0, 0], |pos, dir| {
            for i in 0..2 {
                pos[i] += dir[i];
            }

            Some(*pos)
        })
        .map_windows(|[[x1, y1], [x2, y2]]| {
            [x1 * y2 - x2 * y1, (x1 - x2).abs() + (y1 - y2).abs()]
        })
        .fold([0, 0], |[area, perimeter], [da, dp]| {
            [area + da, perimeter + dp]
        })
        .iter()
        .sum::<i64>()
        / 2
        + 1
}

fn part1(input: &str) {
    let ans = find_area(input.lines().map(|l| {
        let [d, n, _] = l
            .split_ascii_whitespace()
            .collect::<ArrayVec<_, 3>>()
            .into_inner()
            .unwrap();

        let n: i64 = n.parse().unwrap();

        match d {
            "R" => [n, 0],
            "D" => [0, n],
            "L" => [-n, 0],
            "U" => [0, -n],
            _ => panic!(),
        }
    }));

    println!("{ans}");
}

fn part2(input: &str) {
    let ans = find_area(input.lines().map(|l| {
        let [_, _, hex] = l
            .split_ascii_whitespace()
            .collect::<ArrayVec<_, 3>>()
            .into_inner()
            .unwrap();

        let n = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
        let d = hex.chars().nth(hex.len() - 2).unwrap();

        match d {
            '0' => [n, 0],
            '1' => [0, n],
            '2' => [-n, 0],
            '3' => [0, -n],
            _ => panic!(),
        }
    }));

    println!("{ans}");
}
