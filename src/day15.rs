use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |curval, c| ((curval as u16 + c as u16) * 17) as u8)
}

fn part1(input: &str) {
    let input = input.split_ascii_whitespace().next().unwrap();

    let ans: u32 = input.split(',').map(hash).map(|x| x as u32).sum();

    println!("{ans}");
}

fn part2(input: &str) {
    let inp = input.split_ascii_whitespace().next().unwrap();

    let mut boxes = [0; 256].map(|_| Vec::new());

    for entry in inp.split(',') {
        if entry.ends_with('-') {
            let k = entry.split('-').next().unwrap();
            let h = hash(k);
            if let Some(i) = boxes[h as usize]
                .iter()
                .enumerate()
                .find_map(|(i, &(k2, _))| (k == k2).then_some(i))
            {
                boxes[h as usize].remove(i);
            }
        } else {
            let [k, v] = entry
                .split('=')
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            let v = v.parse::<u8>().unwrap();
            let h = hash(k);

            if let Some((_, x)) =
                boxes[h as usize].iter_mut().find(|(k2, _)| k == *k2)
            {
                *x = v;
            } else {
                boxes[h as usize].push((k, v));
            }
        }
    }

    let ans: usize = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, (_, x))| (i + 1) * (j + 1) * x as usize)
        })
        .sum();

    println!("{ans}");
}
