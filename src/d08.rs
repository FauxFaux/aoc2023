use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

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
    let mut i = 0usize;
    let mut moves = moves.iter().cycle();
    let mut here = grid.keys().filter(|k| k.ends_with('A')).collect_vec();
    loop {
        let m = moves.next().expect("move");
        for here in here.iter_mut() {
            let (l, r) = grid.get(*here).expect("here");
            *here = if *m { r } else { l };
        }
        i += 1;
        if i % (1024 * 1024) == 0 {
            println!("{i} {:?}", here);
        }
        if here.iter().all(|k| k.ends_with('Z')) {
            println!("found ZZZ in {} moves", i);
            break;
        }
    }
}
