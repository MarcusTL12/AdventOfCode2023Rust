use std::{
    collections::{HashMap, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

#[derive(Debug, Hash)]
enum ModuleState<'a, const N: usize> {
    FlipFlop(bool),
    Conjunction(ArrayVec<(&'a str, bool), N>),
    Broadcaster,
}

use num_integer::lcm;
use ModuleState::*;

fn parse_input<const IN: usize, const OUT: usize>(
    input: &str,
) -> HashMap<&str, (ModuleState<IN>, ArrayVec<&str, OUT>)> {
    let mut inp_queue = Vec::new();

    let mut map: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let (name, dests) = l.split_once(" -> ").unwrap();

            let t = match name.chars().next().unwrap() {
                'b' => Broadcaster,
                '%' => FlipFlop(false),
                '&' => Conjunction(ArrayVec::new()),
                _ => panic!(),
            };

            let name = match t {
                Broadcaster => name,
                _ => &name[1..],
            };

            let dests = dests
                .split(", ")
                .map(|s| {
                    inp_queue.push((name, s));
                    s
                })
                .collect::<ArrayVec<_, OUT>>();

            (name, (t, dests))
        })
        .collect();

    for (from, to) in inp_queue {
        if let Some((Conjunction(inputs), _)) = map.get_mut(to) {
            inputs.push((from, false));
        }
    }

    map
}

fn pushbutton<'a, const IN: usize, const OUT: usize>(
    modules: &mut HashMap<&str, (ModuleState<IN>, ArrayVec<&'a str, OUT>)>,
    pulsequeue: &mut VecDeque<([&'a str; 2], bool)>,
) -> [usize; 2] {
    pulsequeue.clear();
    pulsequeue.push_back((["button", "broadcaster"], false));

    let mut n_low = 0;
    let mut n_high = 0;

    while let Some(([from, to], pulse)) = pulsequeue.pop_front() {
        *if pulse { &mut n_high } else { &mut n_low } += 1;

        if let Some((state, outputs)) = modules.get_mut(to) {
            match state {
                Broadcaster => {
                    for dest in outputs {
                        pulsequeue.push_back(([to, dest], pulse))
                    }
                }
                FlipFlop(flopstate) => {
                    if !pulse {
                        *flopstate = !*flopstate;
                        for dest in outputs {
                            pulsequeue.push_back(([to, dest], *flopstate))
                        }
                    }
                }
                Conjunction(inputstate) => {
                    if let Some((_, x)) =
                        inputstate.iter_mut().find(|(name, _)| *name == from)
                    {
                        *x = pulse;
                    }

                    let nextpulse = !inputstate.iter().all(|&(_, x)| x);
                    for dest in outputs {
                        pulsequeue.push_back(([to, dest], nextpulse))
                    }
                }
            }
        }
    }

    [n_low, n_high]
}

fn part1(input: &str) {
    let mut modules = parse_input::<10, 8>(input);
    let mut pulsequeue = VecDeque::new();

    let mut n_low = 0;
    let mut n_high = 0;

    for _ in 0..1000 {
        let [l, h] = pushbutton(&mut modules, &mut pulsequeue);

        n_low += l;
        n_high += h;
    }

    let ans = n_low * n_high;

    println!("{ans}");
}

fn partition_input<'a, const IN: usize, const OUT: usize>(
    modules: &HashMap<&str, (ModuleState<IN>, ArrayVec<&'a str, OUT>)>,
) -> Vec<Vec<&'a str>> {
    let &rx_inp = modules
        .iter()
        .find_map(|(k, v)| v.1.contains(&"rx").then_some(k))
        .unwrap();

    let mut stk = Vec::new();

    modules["broadcaster"]
        .1
        .iter()
        .map(|&name| {
            stk.push(name);
            let mut names = Vec::new();
            names.push(name);
            while let Some(name) = stk.pop() {
                for &outname in &modules[name].1 {
                    if outname != rx_inp && !names.contains(&outname) {
                        stk.push(outname);
                        names.push(outname);
                    }
                }
            }
            names
        })
        .collect()
}

fn get_state_hash<const IN: usize, const OUT: usize>(
    modules: &HashMap<&str, (ModuleState<IN>, ArrayVec<&str, OUT>)>,
    partition: &[&str],
) -> u64 {
    let mut s = DefaultHasher::new();

    for name in partition {
        modules[name].0.hash(&mut s);
    }

    s.finish()
}

fn part2(input: &str) {
    let mut modules = parse_input::<10, 8>(input);
    let mut pulsequeue = VecDeque::new();

    let partitions = partition_input(&modules);

    let mut seen_states = vec![HashMap::new(); partitions.len()];
    let mut cycles = vec![None; partitions.len()];

    for i in 0.. {
        if !cycles.iter().any(|x| x.is_none()) {
            break;
        }

        pushbutton(&mut modules, &mut pulsequeue);

        for ((partition, seen), cycle) in
            partitions.iter().zip(&mut seen_states).zip(&mut cycles)
        {
            let state = get_state_hash(&modules, partition);
            if let Some(&j) = seen.get(&state) {
                *cycle = Some((j, i));
            } else {
                seen.insert(state, i);
            }
        }
    }

    let ans: u64 = cycles
        .into_iter()
        .map(|x| x.unwrap())
        .map(|(start, stop)| stop - start)
        .fold(1, lcm);

    println!("{ans}");
}
