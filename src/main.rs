#![feature(iter_array_chunks)]

use home::home_dir;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let funcs = [
        day1::PARTS,
        day2::PARTS,
        day3::PARTS,
        day4::PARTS,
        day5::PARTS,
        day6::PARTS,
        day7::PARTS,
        day8::PARTS,
        day9::PARTS,
        day10::PARTS,
        day11::PARTS,
        day12::PARTS,
        day13::PARTS,
    ];
    let mut args = std::env::args();
    args.next();
    match args.next() {
        Some(x) if x == "all" => {
            println!("Running all days:");
            println!("===========================");
            let timer = std::time::Instant::now();
            for (i, parts) in funcs.iter().enumerate() {
                let path = home_dir()
                    .unwrap()
                    .join("aoc-input/2023/")
                    .join(format!("day{}", i + 1))
                    .join("input");
                let path = path.to_str().unwrap();

                println!("---------------------------");
                println!("Running Day {}", i + 1);
                println!("Part 1:");
                let subtimer = std::time::Instant::now();
                parts[0](path);
                println!("{:?}\n", subtimer.elapsed());

                println!("Part 2:");
                let subtimer = std::time::Instant::now();
                parts[1](path);
                println!("{:?}", subtimer.elapsed());
            }
            println!("===========================");
            println!("Took {:?}", timer.elapsed());
        }
        Some(x) => {
            if let Ok(x) = x.parse::<usize>() {
                if let Some(y) = args.next() {
                    if let Ok(y) = y.parse::<usize>() {
                        if let Some(f) = funcs.get(x - 1) {
                            if let Some(f) = f.get(y - 1) {
                                let path =
                                    args.next().unwrap_or("input".to_owned());
                                let path = home_dir()
                                    .unwrap()
                                    .join("aoc-input/2023/")
                                    .join(format!("day{x}"))
                                    .join(path);
                                let path = path.to_str().unwrap();
                                let timer = std::time::Instant::now();
                                f(path);
                                println!("Took {:?}", timer.elapsed());
                            } else {
                                println!("Not implemented");
                            }
                        } else {
                            println!("Not implemented");
                        }
                    } else {
                        println!("Must enter numbers!");
                    }
                } else {
                    println!("Pass day and part as commandline parameters");
                }
            } else {
                println!("Must enter numbers!");
            }
        }
        _ => println!(concat!(
            "Run specific day with day and part as command ",
            "line arguments, \nor run all days by giving \"all\" as argument"
        )),
    }
}
