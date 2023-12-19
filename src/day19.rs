use std::collections::HashMap;

use arrayvec::ArrayVec;
use regex::Regex;

pub const PARTS: [fn(&str); 2] = [part1, part2];

type Part = [usize; 4];
type PartRange = [[usize; 2]; 4];

#[derive(Clone, Copy)]
enum Category {
    X = 0,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        use Category::*;
        match value {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!(),
        }
    }
}

enum Dir {
    Less,
    Greater,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        use Dir::*;
        match value {
            "<" => Less,
            ">" => Greater,
            _ => panic!(),
        }
    }
}

struct Rule<'a> {
    category: Category,
    val: usize,
    dir: Dir,
    dest: &'a str,
}

impl Rule<'_> {
    fn map(&self, part: Part) -> Option<&str> {
        let val = part[self.category as usize];

        match self.dir {
            Dir::Less => val < self.val,
            Dir::Greater => val > self.val,
        }
        .then_some(self.dest)
    }

    fn map_range(&self, part_range: PartRange) -> [Option<PartRange>; 2] {
        let r = part_range[self.category as usize];

        let [r_mapped, r_rest] = match self.dir {
            Dir::Less => {
                [[r[0], r[1].min(self.val - 1)], [r[0].max(self.val), r[1]]]
            }
            Dir::Greater => {
                [[r[0].max(self.val + 1), r[1]], [r[0], r[1].min(self.val)]]
            }
        };

        let make_part_range = |r: [usize; 2]| {
            (r[0] <= r[1]).then(|| {
                let mut new_range = part_range;
                new_range[self.category as usize] = r;
                new_range
            })
        };

        [make_part_range(r_mapped), make_part_range(r_rest)]
    }
}

struct Workflow<'a, const N: usize> {
    rules: ArrayVec<Rule<'a>, N>,
    last: &'a str,
}

impl<const N: usize> Workflow<'_, N> {
    fn map(&self, part: Part) -> &str {
        self.rules
            .iter()
            .find_map(|rule| rule.map(part))
            .unwrap_or(self.last)
    }
}

fn parse_input<const N: usize>(
    input: &str,
) -> (HashMap<&str, Workflow<N>>, impl Iterator<Item = &str>) {
    let reg = Regex::new(r"(\w)(<|>)(\d+):(\w+)").unwrap();

    let mut lines = input.lines();

    let workflows = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let [name, rest] = line
                .split('{')
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            let mut last = "";

            let rules = rest[0..rest.len() - 1]
                .split(',')
                .filter_map(|s| {
                    last = s;

                    reg.captures(s).map(|c| {
                        let category = Category::from(&c[1]);
                        let val = c[3].parse().unwrap();
                        let dir = Dir::from(&c[2]);
                        let dest = c.get(4).unwrap().as_str();

                        Rule {
                            category,
                            val,
                            dir,
                            dest,
                        }
                    })
                })
                .collect();

            (name, Workflow { rules, last })
        })
        .collect();

    (workflows, lines)
}

fn is_accepted<const N: usize>(
    workflows: &HashMap<&str, Workflow<N>>,
    part: Part,
) -> bool {
    let mut id = "in";

    loop {
        id = workflows[id].map(part);

        if id == "A" {
            break true;
        } else if id == "R" {
            break false;
        }
    }
}

fn part1(input: &str) {
    let (workflows, parts) = parse_input::<4>(input);

    let ans: usize = parts
        .map(|part| {
            part[1..part.len() - 1]
                .split(',')
                .map(|s| s.split('=').nth(1).unwrap().parse().unwrap())
                .collect::<ArrayVec<_, 4>>()
                .into_inner()
                .unwrap()
        })
        .filter(|&part| is_accepted(&workflows, part))
        .map(|part| part.iter().sum::<usize>())
        .sum();

    println!("{ans}");
}

fn num_accepted<const N: usize>(
    workflows: &HashMap<&str, Workflow<N>>,
    id: &str,
    part: PartRange,
) -> usize {
    if id == "A" {
        return part.iter().map(|r| r[1] - r[0] + 1).product();
    } else if id == "R" {
        return 0;
    }

    let workflow = &workflows[id];
    let mut part = Some(part);
    workflow
        .rules
        .iter()
        .filter_map(|rule| {
            part.and_then(|part_range| {
                let [mapped_range, rest_range] = rule.map_range(part_range);
                part = rest_range;

                mapped_range
                    .map(|range| num_accepted(workflows, rule.dest, range))
            })
        })
        .sum::<usize>()
        + part
            .map(|part_range| {
                num_accepted(workflows, workflow.last, part_range)
            })
            .unwrap_or(0)
}

fn part2(input: &str) {
    let (workflows, _) = parse_input::<4>(input);

    let ans = num_accepted(&workflows, "in", [[1, 4000]; 4]);

    println!("{ans}");
}
