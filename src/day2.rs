use regex::Regex;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn part1(input: &str) {
    let reg = Regex::new(r"(\d+) (\w+)").unwrap();

    let ans: usize = input
        .lines()
        .enumerate()
        .filter_map(|(id, l)| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for c in reg.captures_iter(l) {
                let n: usize = c[1].parse().unwrap();

                let p = match &c[2] {
                    "red" => &mut r,
                    "green" => &mut g,
                    "blue" => &mut b,
                    _ => panic!(),
                };

                *p = n.max(*p);
            }

            (r <= 12 && g <= 13 && b <= 14).then_some(id + 1)
        })
        .sum();

    println!("{}", ans);
}

fn part2(input: &str) {
    let reg = Regex::new(r"(\d+) (\w+)").unwrap();

    let ans: usize = input
        .lines()
        .map(|l| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for c in reg.captures_iter(l) {
                let n: usize = c[1].parse().unwrap();

                let p = match &c[2] {
                    "red" => &mut r,
                    "green" => &mut g,
                    "blue" => &mut b,
                    _ => panic!(),
                };

                *p = n.max(*p);
            }

            r * b * g
        })
        .sum();

    println!("{}", ans);
}
