use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn part1(input: &str) {
    let mut buf = Vec::new();

    let ans: u64 = input
        .lines()
        .map(|l| {
            let l = l.split(": ").nth(1).unwrap();
            let [first, second] = l
                .split(" | ")
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            buf.clear();
            buf.extend(
                first.split_ascii_whitespace().map(|s| s.parse::<u64>()),
            );

            let n = second
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>())
                .filter(|x| buf.contains(x))
                .count();

            if n > 0 {
                2u64.pow((n - 1) as u32)
            } else {
                0
            }
        })
        .sum();

    println!("{ans}");
}

fn part2(input: &str) {
    let mut buf = Vec::new();

    let mut n_cards = Vec::new();

    for (i, l) in input.lines().enumerate() {
        let l = l.split(": ").nth(1).unwrap();
        let [first, second] = l
            .split(" | ")
            .collect::<ArrayVec<_, 2>>()
            .into_inner()
            .unwrap();

        buf.clear();
        buf.extend(first.split_ascii_whitespace().map(|s| s.parse::<u64>()));

        let n = second
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>())
            .filter(|x| buf.contains(x))
            .count();

        let endind = i + n + 1;
        if endind > n_cards.len() {
            n_cards.resize(endind, 1);
        }

        let n_cards_pos = n_cards[i];

        for x in &mut n_cards[i + 1..endind] {
            *x += n_cards_pos;
        }
    }

    let ans: usize = n_cards.into_iter().sum();

    println!("{ans}");
}
