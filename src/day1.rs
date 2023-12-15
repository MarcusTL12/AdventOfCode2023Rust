pub const PARTS: [fn(&str); 2] = [part1, part2];

fn part1(input: &str) {
    let ans: u64 = input
        .lines()
        .map(|l| {
            let d1 = l.chars().find(|c| c.is_numeric()).unwrap() as u8 - b'0';
            let d2 = l.chars().filter(|c| c.is_numeric()).last().unwrap() as u8
                - b'0';

            (d1 * 10 + d2) as u64
        })
        .sum();

    println!("{}", ans);
}

fn part2(input: &str) {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let ans: u64 = input
        .lines()
        .map(|l| {
            let d1 = (0..=l.len())
                .find_map(|i| {
                    (1..=9).find(|j| {
                        l[i..].starts_with((j + b'0') as char)
                            || l[i..].starts_with(numbers[*j as usize - 1])
                    })
                })
                .unwrap();

            let d2 = (0..=l.len())
                .rev()
                .find_map(|i| {
                    (1..=9).find(|j| {
                        l[..i].ends_with((j + b'0') as char)
                            || l[..i].ends_with(numbers[*j as usize - 1])
                    })
                })
                .unwrap();

            (d1 * 10 + d2) as u64
        })
        .sum();

    println!("{}", ans);
}
