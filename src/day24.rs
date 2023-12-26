use arrayvec::ArrayVec;
use num_rational::Ratio;
use num_traits::Signed;

pub const PARTS: [fn(&str); 2] = [part1, part2];

fn parse_input(input: &str) -> Vec<[[i64; 3]; 2]> {
    input
        .lines()
        .map(|l| {
            l.split(" @ ")
                .map(|p| {
                    p.split(", ")
                        .map(|x| x.parse().unwrap())
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

fn part1(input: &str) {
    type R = Ratio<i128>;

    // (x, y) = (x0, y0) + (vx, vy) * t
    // x = x0 + vx * t
    // t = (x - x0) / vx
    // y = y0 + vy * t
    // y = y0 + (x - x0) * vy / vx
    // y = (vy / vx) * x - (vy / vx) * x0 + y0
    // a = vy / vx
    // b = y0 - a * x0

    // y = a1 x + b1
    // y = a2 x + b2
    // (a2 - a1) x + b2 - b1 = 0
    // x = (b1 - b2) / (a2 - a1)
    // y = a1 x + b1

    let data = parse_input(input);

    const BOXMIN: R = R::new_raw(200000000000000, 1);
    const BOXMAX: R = R::new_raw(400000000000000, 1);

    let ans = data
        .iter()
        .enumerate()
        .flat_map(|(i, &[[x1, y1, _], [vx1, vy1, _]])| {
            data.iter()
                .take(i)
                .filter(move |&&[[x2, y2, _], [vx2, vy2, _]]| {
                    let a1 = R::new(vy1 as i128, vx1 as i128);
                    let a2 = R::new(vy2 as i128, vx2 as i128);

                    let b1 = R::from(y1 as i128) - a1 * R::from(x1 as i128);
                    let b2 = R::from(y2 as i128) - a2 * R::from(x2 as i128);

                    if a1 == a2 {
                        false
                    } else {
                        let x = (b1 - b2) / (a2 - a1);
                        let y = a1 * x + b1;

                        let t1 = (x - x1 as i128) / vx1 as i128;
                        let t2 = (x - x2 as i128) / vx2 as i128;

                        t1.is_positive()
                            && t2.is_positive()
                            && (BOXMIN..=BOXMAX).contains(&x)
                            && (BOXMIN..=BOXMAX).contains(&y)
                    }
                })
        })
        .count();

    println!("{ans}");
}

fn part2(_input: &str) {}
