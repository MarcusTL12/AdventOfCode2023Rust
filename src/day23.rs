use arrayvec::ArrayVec;
use ndarray::{Array2, ArrayView2, ArrayViewMut2};

use crate::parse_grid::parse_grid;

pub const PARTS: [fn(&str); 2] = [part1, part2];

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn build_graph<const N: usize>(
    grid: ArrayView2<u8>,
    mut visited: ArrayViewMut2<bool>,
    nodes: &mut Vec<([usize; 2], ArrayVec<[usize; 2], N>)>,
    pos: [usize; 2],
    mut prev_node: usize,
    mut dist_from_prev_node: usize,
    check_open_fn: impl Fn(Option<u8>, usize) -> bool + Copy,
) {
    if let (Some(i), Some(node)) = (
        nodes
            .iter()
            .enumerate()
            .find_map(|(i, (nodepos, _))| (*nodepos == pos).then_some(i)),
        nodes.get_mut(prev_node),
    ) {
        if i != prev_node {
            node.1.push([i, dist_from_prev_node]);
        }
    } else if !visited[pos] {
        visited[pos] = true;

        if DIRS
            .iter()
            .filter(|[dy, dx]| {
                let npos = [
                    (pos[0] as isize + dy) as usize,
                    (pos[1] as isize + dx) as usize,
                ];

                grid.get(npos).unwrap_or(&b'#') != &b'#'
            })
            .count()
            != 2
        {
            nodes.push((pos, ArrayVec::new()));
            let next_node_ind = nodes.len() - 1;
            if nodes.len() > 1 {
                nodes[prev_node]
                    .1
                    .push([next_node_ind, dist_from_prev_node]);
            }

            dist_from_prev_node = 0;
            prev_node = next_node_ind;
        }

        for (i, [dy, dx]) in DIRS.iter().enumerate() {
            let npos = [
                (pos[0] as isize + dy) as usize,
                (pos[1] as isize + dx) as usize,
            ];

            if check_open_fn(grid.get(npos).cloned(), i) {
                build_graph(
                    grid,
                    visited.view_mut(),
                    nodes,
                    npos,
                    prev_node,
                    dist_from_prev_node + 1,
                    check_open_fn,
                );
            }
        }
    }
}

fn dfs_max<const N: usize>(
    nodes: &[([usize; 2], ArrayVec<[usize; 2], N>)],
    visited: &mut [bool],
    target: usize,
    curnode: usize,
    len: usize,
) -> usize {
    if curnode == target {
        len
    } else {
        visited[curnode] = true;

        let mut maxlen = 0;

        for &[othernode, dist] in nodes[curnode].1.iter() {
            if !visited[othernode] {
                maxlen = maxlen.max(dfs_max(
                    nodes,
                    visited,
                    target,
                    othernode,
                    len + dist,
                ));
            }
        }

        visited[curnode] = false;

        maxlen
    }
}

fn part1(input: &str) {
    let grid = parse_grid(input);
    let mut visited = Array2::from_shape_simple_fn(grid.dim(), || false);
    let mut nodes = Vec::new();

    build_graph::<4>(
        grid,
        visited.view_mut(),
        &mut nodes,
        [0, 1],
        0,
        0,
        |c, i| {
            matches!(
                (c, i),
                (Some(b'.'), _)
                    | (Some(b'v'), 0)
                    | (Some(b'^'), 1)
                    | (Some(b'>'), 2)
                    | (Some(b'<'), 3)
            )
        },
    );

    let (h, w) = grid.dim();

    let target = nodes
        .iter()
        .enumerate()
        .find_map(|(i, &(pos, _))| (pos == [h - 1, w - 2]).then_some(i))
        .unwrap();

    let n_nodes = nodes.len();

    let ans = dfs_max(&nodes, &mut vec![false; n_nodes], target, 0, 0);

    println!("{ans}");
}

fn complete_graph<const N: usize>(
    nodes: &mut [([usize; 2], ArrayVec<[usize; 2], N>)],
) {
    for i in 0..nodes.len() {
        for [j, dist] in nodes[i].1.clone() {
            let k = [i, dist];
            if !nodes[j].1.contains(&k) {
                nodes[j].1.push(k);
            }
        }
    }
}

fn part2(input: &str) {
    let grid = parse_grid(input);
    let mut visited = Array2::from_shape_simple_fn(grid.dim(), || false);
    let mut nodes = Vec::new();

    build_graph::<8>(
        grid,
        visited.view_mut(),
        &mut nodes,
        [0, 1],
        0,
        0,
        |c, _| c.unwrap_or(b'#') != b'#',
    );

    complete_graph(&mut nodes);

    let (h, w) = grid.dim();

    let target = nodes
        .iter()
        .enumerate()
        .find_map(|(i, &(pos, _))| (pos == [h - 1, w - 2]).then_some(i))
        .unwrap();

    let n_nodes = nodes.len();

    let ans = dfs_max(&nodes, &mut vec![false; n_nodes], target, 0, 0);

    println!("{ans}");
}
