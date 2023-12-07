use itertools::Itertools;
use maplit::hashset;
use std::collections::HashSet;

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
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).expect("digit") as i64,
                })
                .collect_vec();
            (cs, p.parse::<i64>().expect("bid"))
        })
        .collect_vec();

    gs.sort_by(|(l, pl), (r, pr)| {
        let cl = l.iter().counts();
        let cr = r.iter().counts();

        let clm = *cl.values().max().expect("max");
        let crm = *cr.values().max().expect("max");

        // if clm != crm && clm > 3 {
        //     println!("{l:?} {r:?} max: {clm} {crm}");
        //     return clm.cmp(&crm);
        // }

        let hr = |hc: &[usize]| {
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
            println!("non-hand: {hc:?}");
            0
        };

        let ccl = cl.values().copied().sorted().collect_vec();
        let ccr = cr.values().copied().sorted().collect_vec();

        let hrl = hr(&ccl);
        let hrc = hr(&ccr);
        println!("{l:?} {r:?} {hrl} {hrc} {ccl:?} {ccr:?}");
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
