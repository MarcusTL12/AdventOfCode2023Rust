use std::collections::{HashMap, VecDeque};

use arrayvec::ArrayVec;

pub const PARTS: [fn(&str); 2] = [part1, part2];

#[derive(Debug)]
enum ModuleState<'a, const N: usize> {
    FlipFlop(bool),
    Conjunction(ArrayVec<(&'a str, bool), N>),
    Broadcaster,
}

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

fn part2(_input: &str) {}
