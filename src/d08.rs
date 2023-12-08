use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let mut input = include_str!("d08.txt").lines();
    let moves = input
        .next()
        .expect("moves")
        .chars()
        .map(|c| match c {
            'L' => false,
            'R' => true,
            _ => unreachable!("char"),
        })
        .collect_vec();

    input.next().expect("blank");

    let mut grid = HashMap::with_capacity(1000);
    let reg = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).expect("regex");
    for line in input {
        let ma = reg.captures(line).expect("match");
        let a = ma.get(1).expect("1").as_str();
        let l = ma.get(2).expect("2").as_str();
        let r = ma.get(3).expect("3").as_str();
        grid.insert(a, (l, r));
    }
    grid.shrink_to_fit();

    println!("{:?}", grid);
    let mut nums = Vec::new();
    for start in grid.keys().filter(|k| k.ends_with('A')).collect_vec() {
        let mut moves = moves.iter().cycle().peekable();
        let mut i = 0usize;
        println!("start: {}", start);
        let mut ends_seen = HashSet::new();
        let mut here = start;
        let mut poses = Vec::new();
        loop {
            let m = moves.next().expect("move");
            let (l, r) = grid.get(here).expect("here");
            here = if *m { r } else { l };
            i += 1;
            if here.ends_with('Z') {
                println!("{i} {here} {ends_seen:?} {:?}", moves.peek());

                poses.push(i);
                if !ends_seen.insert((moves.peek().copied(), here)) {
                    break;
                }
            }
        }

        assert_eq!(2, poses.len());
        let (a, b) = (poses[0], poses[1]);
        nums.push((a, b));
    }

    println!("{nums:?}");
    println!(
        "{:?}",
        nums.iter()
            .map(|(a, b)| (*b as f64) / (*a as f64))
            .collect_vec()
    );

    // oh, they're all perfect cycles; just LCM the 'a's
}

// LTA = (KLS, BFV)
// BKZ = (BFV, KLS)
// QGA = (CGR, SGM)
