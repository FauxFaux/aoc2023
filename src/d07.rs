use itertools::Itertools;

pub fn solve() {
    let mut gs = include_str!("d07.txt")
        .lines()
        .map(|l| {
            let (cs, p) = l.split_once(' ').expect("line");
            let cs = cs
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 0,
                    'T' => 10,
                    _ => c.to_digit(10).expect("digit") as i64,
                })
                .collect_vec();
            (cs, p.parse::<i64>().expect("bid"))
        })
        .collect_vec();

    gs.sort_by(|(l, _pl), (r, _pr)| {
        fn hr(h: &[i64]) -> i32 {
            let c = h.iter().counts();
            let hc = c.values().copied().sorted().collect_vec();

            if hc == &[5] {
                return 6;
            }
            if hc == &[1, 4] {
                return 5;
            }
            if hc == &[2, 3] {
                return 4;
            }
            if hc == &[1, 1, 3] {
                return 3;
            }
            if hc == &[1, 2, 2] {
                return 2;
            }
            if hc == &[1, 1, 1, 2] {
                return 1;
            }
            0
        }

        fn hrw(h: &[i64]) -> i32 {
            let mut wilds = h
                .iter()
                .enumerate()
                .filter_map(|(i, p)| (*p == 0).then_some(i))
                .collect_vec();
            if wilds.is_empty() {
                return hr(h);
            }

            let alts = [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14];

            let init = wilds.pop().expect("checked");
            let mut best = 0;
            for a in alts {
                let mut h = h.to_vec();
                h[init] = a;
                let u = hrw(&h);
                if u > best {
                    best = u;
                }
            }

            best
        }

        let hrl = hrw(&l);
        let hrc = hrw(&r);

        println!("{l:?} {r:?} {hrl} {hrc}");
        let lr = hrl.cmp(&hrc);
        if lr != std::cmp::Ordering::Equal {
            return lr;
        }

        let (npl, npr) = l
            .iter()
            .zip(r.iter())
            .find(|(lc, rc)| *lc != *rc)
            .expect("non-equal");
        npl.cmp(npr)
    });

    println!("{gs:?}");

    let sum: i64 = gs
        .iter()
        .enumerate()
        .map(|(i, (_, p))| (i as i64 + 1) * p)
        .sum();

    // too low: 253816004
    //          253910319
    println!("{}", sum);
}
