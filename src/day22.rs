use arrayvec::ArrayVec;
use ndarray::{Array2, Array3, ArrayView3, ArrayViewMut3, Axis};

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> Vec<[[usize; 3]; 2]> {
    input
        .lines()
        .map(|l| {
            l.split('~')
                .map(|s| {
                    s.split(',')
                        .map(|n| n.parse().unwrap())
                        .collect::<ArrayVec<_, 3>>()
                        .into_inner()
                        .unwrap()
                })
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap()
        })
        .collect()
}

fn reverse_array<const N: usize, T>(mut a: [T; N]) -> [T; N] {
    a.reverse();
    a
}

fn get_occupancy_grid(bricks: &[[[usize; 3]; 2]]) -> Array3<Option<usize>> {
    let dim = bricks.iter().map(|[_, xyz]| xyz).fold(
        [1; 3],
        |[wx, wy, wz], &[x, y, z]| {
            [wx.max(x + 1), wy.max(y + 1), wz.max(z + 1)]
        },
    );

    let mut occ = Array3::from_shape_simple_fn(reverse_array(dim), || None);

    for (i, &[[x1, y1, z1], [x2, y2, z2]]) in bricks.iter().enumerate() {
        for z in [z1, z2] {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    occ[[z, y, x]] = Some(i)
                }
            }
        }
    }

    occ
}

fn fall(bricks: &mut [[[usize; 3]; 2]], mut occ: ArrayViewMut3<Option<usize>>) {
    let (wz, wy, wx) = occ.dim();

    let mut first_free_spaces = Array2::from_shape_simple_fn([wy, wx], || 1);

    let mut has_fallen = vec![false; bricks.len()];

    for z in 0..wz {
        for y in 0..wy {
            for x in 0..wx {
                if let Some(i) = occ[[z, y, x]] {
                    let [[x1, y1, _], [x2, y2, z2]] = bricks[i];
                    if !has_fallen[i] {
                        has_fallen[i] = true;
                        let first_free = (y1..=y2)
                            .flat_map(|y| {
                                (x1..=x2)
                                    .map(move |x| [y, x])
                                    .map(|ind| first_free_spaces[ind])
                            })
                            .max()
                            .unwrap();

                        for y in y1..=y2 {
                            for x in x1..=x2 {
                                occ[[z, y, x]] = None;
                                occ[[z2, y, x]] = None;

                                occ[[first_free, y, x]] = Some(i);
                                occ[[first_free + z2 - z, y, x]] = Some(i);

                                first_free_spaces[[y, x]] =
                                    first_free + z2 - z + 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn build_tree<const N: usize>(
    bricks: &[[[usize; 3]; 2]],
    occ: ArrayView3<Option<usize>>,
) -> Vec<[ArrayVec<usize, N>; 2]> {
    let mut tree = vec![[ArrayVec::new(), ArrayVec::new()]; bricks.len()];

    for window in occ.axis_windows(Axis(0), 2) {
        let [below, above] = window
            .outer_iter()
            .collect::<ArrayVec<_, 2>>()
            .into_inner()
            .unwrap();

        if above.iter().all(|x| x.is_none()) {
            break;
        }

        for (b, a) in below.iter().zip(above.iter()) {
            if let (&Some(b), &Some(a)) = (b, a) {
                if a != b && !tree[b][1].contains(&a) {
                    tree[b][1].push(a);
                    tree[a][0].push(b);
                }
            }
        }
    }

    tree
}

fn part1(input: &str) {
    let mut bricks = parse_input(input);
    let mut occ = get_occupancy_grid(&bricks);
    fall(&mut bricks, occ.view_mut());
    let tree = build_tree::<4>(&bricks, occ.view());

    let ans = tree
        .iter()
        .filter(|[_, above]| above.iter().all(|&i| tree[i][0].len() > 1))
        .count();

    println!("{ans}");
}

fn count_falling<const N: usize>(
    standing: &mut [bool],
    tree: &[[ArrayVec<usize, N>; 2]],
    i: usize,
) -> usize {
    assert_eq!(standing.len(), tree.len());

    if standing[i] {
        standing[i] = false;

        let [_, above] = &tree[i];

        let mut n = 0;

        for &j in above {
            if standing[j]
                && tree[j][0].iter().filter(|&&k| standing[k]).count() == 0
            {
                n += 1 + count_falling(standing, tree, j);
            }
        }

        n
    } else {
        0
    }
}

fn part2(input: &str) {
    let mut bricks = parse_input(input);
    let mut occ = get_occupancy_grid(&bricks);
    fall(&mut bricks, occ.view_mut());
    let tree = build_tree::<4>(&bricks, occ.view());

    let mut standing = vec![true; tree.len()];

    let ans: usize = (0..tree.len())
        .map(|i| {
            standing.fill(true);
            count_falling(&mut standing, &tree, i)
        })
        .sum();

    println!("{ans}");
}
