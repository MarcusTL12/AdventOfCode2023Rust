use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn shift_pos([y, x]: [usize; 2], dir: usize) -> [usize; 2] {
    match dir {
        0 => [y, x + 1],
        1 => [y + 1, x],
        2 => [y, x.wrapping_sub(1)],
        3 => [y.wrapping_sub(1), x],
        _ => panic!(),
    }
}

fn part1(input: &str) {
    let grid = parse_grid(input);

    let (h, w) = grid.dim();

    let mandist = |[y, x]: [usize; 2]| y.abs_diff(h - 1) + x.abs_diff(w - 1);

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();

    for k in [([0, 0], 0, 0), ([0, 0], 1, 0)] {
        queue.push_decrease(k, Reverse(mandist([0, 0])));
        seen.insert(k);
    }

    while let Some(((pos, dir, nstraight), heatdist)) = queue.pop() {
        seen.insert((pos, dir, nstraight));
        let heatloss = heatdist.0 - mandist(pos);

        if pos == [h - 1, w - 1] {
            println!("{heatloss}");
            break;
        }

        if nstraight < 3 {
            let npos = shift_pos(pos, dir);
            let k = (npos, dir, nstraight + 1);
            if let (false, Some(c)) = (seen.contains(&k), grid.get(npos)) {
                queue.push_increase(
                    k,
                    Reverse(heatloss + (c - b'0') as usize + mandist(npos)),
                );
            }
        }

        for dirchange in [1, 3] {
            let ndir = (dir + dirchange) % 4;
            let npos = shift_pos(pos, ndir);
            let k = (npos, ndir, 1);
            if let (false, Some(c)) = (seen.contains(&k), grid.get(npos)) {
                queue.push_increase(
                    k,
                    Reverse(heatloss + (c - b'0') as usize + mandist(npos)),
                );
            }
        }
    }
}

fn part2(input: &str) {
    let grid = parse_grid(input);

    let (h, w) = grid.dim();

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();

    let mandist = |[y, x]: [usize; 2]| y.abs_diff(h - 1) + x.abs_diff(w - 1);

    for k in [([0, 0], 0, 0), ([0, 0], 1, 0)] {
        queue.push_decrease(k, Reverse(mandist([0, 0])));
        seen.insert(k);
    }

    while let Some(((pos, dir, nstraight), heatdist)) = queue.pop() {
        seen.insert((pos, dir, nstraight));
        let heatloss = heatdist.0 - mandist(pos);

        if pos == [h - 1, w - 1] && nstraight >= 4 {
            println!("{heatloss}");
            break;
        }

        if nstraight < 10 {
            let npos = shift_pos(pos, dir);
            let k = (npos, dir, nstraight + 1);
            if let (false, Some(c)) = (seen.contains(&k), grid.get(npos)) {
                queue.push_increase(
                    k,
                    Reverse(heatloss + (c - b'0') as usize + mandist(npos)),
                );
            }
        }

        if nstraight >= 4 {
            for dirchange in [1, 3] {
                let ndir = (dir + dirchange) % 4;
                let npos = shift_pos(pos, ndir);
                let k = (npos, ndir, 1);
                if let (false, Some(c)) = (seen.contains(&k), grid.get(npos)) {
                    queue.push_increase(
                        k,
                        Reverse(heatloss + (c - b'0') as usize + mandist(npos)),
                    );
                }
            }
        }
    }
}
