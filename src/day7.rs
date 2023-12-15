use std::cmp::Reverse;

use arrayvec::ArrayVec;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub const PARTS: [fn(&str); 2] = [part1, part2];

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn tocardnum(c: char) -> Card {
    FromPrimitive::from_usize(match c {
        '2'..='9' => c as usize - b'0' as usize,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    })
    .unwrap()
}

fn find_hand_counts(hand: [Card; 5]) -> [usize; 13] {
    let mut counts = [0; 13];

    for c in hand {
        counts[c as usize - 2] += 1;
    }

    counts
}

fn find_hand_type(counts: [usize; 13]) -> usize {
    if counts.contains(&5) {
        7
    } else if counts.contains(&4) {
        6
    } else if counts.contains(&3) {
        if counts.contains(&2) {
            5
        } else {
            4
        }
    } else if counts.contains(&2) {
        if counts.into_iter().filter(|&x| x == 2).count() == 2 {
            3
        } else {
            2
        }
    } else {
        1
    }
}

fn part1(input: &str) {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let [hand_str, bet_str] = l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            let hand = hand_str
                .chars()
                .map(tocardnum)
                .collect::<ArrayVec<_, 5>>()
                .into_inner()
                .unwrap();

            let bet: u64 = bet_str.parse().unwrap();

            let hand_type = find_hand_type(find_hand_counts(hand));

            (hand_type, hand, bet)
        })
        .collect();

    hands.sort();

    let ans: u64 = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, b))| (i as u64 + 1) * b)
        .sum();

    println!("{ans}");
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Card2 {
    Jack = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

fn tocardnum2(c: char) -> Card2 {
    FromPrimitive::from_usize(match c {
        '2'..='9' => c as usize - b'0' as usize,
        'T' => 10,
        'J' => 1,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!(),
    })
    .unwrap()
}

fn find_hand_counts2(hand: [Card2; 5]) -> [usize; 13] {
    let mut counts = [0; 13];

    for c in hand {
        counts[c as usize - 1] += 1;
    }

    counts
}

fn find_hand_type2(hand: [Card2; 5]) -> usize {
    let mut counts = find_hand_counts2(hand);
    let n_j = counts[0];
    counts[0] = 0;
    counts.sort_by_key(|&x| Reverse(x));

    fn find_type_rec(n_j: usize, counts: &mut [usize; 13]) -> usize {
        let mut best = find_hand_type(*counts);

        if n_j > 0 {
            for i in 0..2 {
                counts[i] += 1;
                best = best.max(find_type_rec(n_j - 1, counts));
                counts[i] -= 1;
            }
        }

        best
    }

    find_type_rec(n_j, &mut counts)
}

fn part2(input: &str) {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let [hand_str, bet_str] = l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            let hand = hand_str
                .chars()
                .map(tocardnum2)
                .collect::<ArrayVec<_, 5>>()
                .into_inner()
                .unwrap();

            let bet: u64 = bet_str.parse().unwrap();

            let hand_type = find_hand_type2(hand);

            (hand_type, hand, bet)
        })
        .collect();

    hands.sort();

    let ans: u64 = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, b))| (i as u64 + 1) * b)
        .sum();

    println!("{ans}");
}
