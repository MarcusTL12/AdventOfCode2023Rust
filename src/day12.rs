use std::collections::HashMap;

use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn find_num_ways(
    memo: &mut HashMap<[usize; 3], usize>,
    line: &[u8],
    nums: &[usize],
    i: usize,
    groupsize: usize,
    j: usize,
) -> usize {
    if i >= line.len() {
        return if j == nums.len() - 1 && groupsize == nums[j]
            || j == nums.len() && groupsize == 0
        {
            1
        } else {
            0
        };
    }

    let k = [i, groupsize, j];

    if let Some(&x) = memo.get(&k) {
        return x;
    }

    let a = if matches!(line[i], b'#' | b'?')
        && groupsize < nums.get(j).cloned().unwrap_or(0)
    {
        find_num_ways(memo, line, nums, i + 1, groupsize + 1, j)
    } else {
        0
    };

    let b = if matches!(line[i], b'.' | b'?') {
        if let Some(next_j) = match groupsize {
            0 => Some(j),
            x if x == nums[j] => Some(j + 1),
            _ => None,
        } {
            find_num_ways(memo, line, nums, i + 1, 0, next_j)
        } else {
            0
        }
    } else {
        0
    };

    memo.insert(k, a + b);

    a + b
}

fn part1(input: &str) {
    let mut nums_buf = Vec::new();
    let mut memo = HashMap::new();

    let ans: usize = input
        .lines()
        .map(|l| {
            let [line, nums] = l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            nums_buf.clear();
            nums_buf
                .extend(nums.split(',').map(|x| x.parse::<usize>().unwrap()));

            memo.clear();
            find_num_ways(&mut memo, line.as_bytes(), &nums_buf, 0, 0, 0)
        })
        .sum();

    println!("{ans}");
}

fn part2(input: &str) {
    let mut nums_buf = Vec::new();
    let mut nums_buf2 = Vec::new();
    let mut linebuf = Vec::new();
    let mut memo = HashMap::new();

    let ans: usize = input
        .lines()
        .map(|l| {
            let [line, nums] = l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            nums_buf.clear();
            nums_buf
                .extend(nums.split(',').map(|x| x.parse::<usize>().unwrap()));

            nums_buf2.clear();
            for _ in 0..5 {
                nums_buf2.extend(nums_buf.iter().cloned());
            }

            linebuf.clear();
            linebuf.extend(line.as_bytes());
            for _ in 0..4 {
                linebuf.push(b'?');
                linebuf.extend(line.as_bytes());
            }

            memo.clear();
            find_num_ways(&mut memo, &linebuf, &nums_buf2, 0, 0, 0)
        })
        .sum();

    println!("{ans}");
}
