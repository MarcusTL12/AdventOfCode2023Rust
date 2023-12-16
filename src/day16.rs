pub const PARTS: [fn(&str); 2] = [part1, part2];

#[derive(Debug)]
enum Reflector {
    MirrorRight,
    MirrorLeft,
    SplitHorizontal,
    SplitVertical,
}

impl From<char> for Reflector {
    fn from(value: char) -> Self {
        use Reflector::*;
        match value {
            '\\' => MirrorRight,
            '/' => MirrorLeft,
            '-' => SplitHorizontal,
            '|' => SplitVertical,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Node {
    pub reflector: Reflector,
    pub pos: [usize; 2],
    pub connections: [Option<usize>; 4],
}

type Edge = [Option<usize>; 2];

fn parse_input(input: &str) -> (Vec<Node>, [Vec<Edge>; 2]) {
    let mut top = Vec::new();

    let mut nodes = Vec::<Node>::new();

    let mut hlines = Vec::new();
    let mut vlines = Vec::new();

    for (y, l) in input.lines().enumerate() {
        if top.is_empty() {
            top.resize(l.len(), None);
        }

        let mut left = None;

        for (x, c) in l.chars().enumerate().filter(|&(_, c)| c != '.') {
            let node = Node {
                reflector: Reflector::from(c),
                pos: [x, y],
                connections: [left, top[x], None, None],
            };
            let i = Some(nodes.len());

            hlines.push([left, i]);
            vlines.push([top[x], i]);

            if let Some(j) = left {
                nodes[j].connections[2] = i;
            }

            if let Some(j) = top[x] {
                nodes[j].connections[3] = i;
            }

            left = i;
            top[x] = i;
            nodes.push(node);
        }
    }

    (nodes, [hlines, vlines])
}

fn part1(input: &str) {
    let (nodes, [hlines, vlines]) = parse_input(input);

    println!("{} {} {}", nodes.len(), hlines.len(), vlines.len());

    // println!("{nodes:#?}");
    // println!("\n{hlines:?}");
    // println!("\n{vlines:?}");
}

fn part2(input: &str) {
    parse_input(input);
}
