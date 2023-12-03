use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let reg_nums = regex::bytes::Regex::new("\\d+").unwrap();

    let g = include_str!("d03.txt")
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    let w = g[0].len();
    let h = g.len();

    let mut nums = Vec::new();
    let mut syms = Vec::new();
    let mut adj = HashMap::new();

    for (y, l) in g.iter().enumerate() {
        for c in reg_nums.find_iter(l) {
            nums.push((
                String::from_utf8_lossy(c.as_bytes())
                    .parse::<usize>()
                    .unwrap(),
                c.range(),
                y,
            ));
        }

        for s in l
            .iter()
            .enumerate()
            .filter_map(|(p, c)| b"*".contains(c).then_some(p))
        {
            syms.push((s, y));
        }
    }

    let mut sum = 0;

    for (n, xs, y) in &nums {
        let lx = xs.start.saturating_sub(1);
        let rx = (xs.end - 1).saturating_add(1);
        let ly = y.saturating_sub(1);
        let ry = y.saturating_add(1);
        for s in syms
            .iter()
            .filter(|(x, y)| *x >= lx && *x <= rx && *y >= ly && *y <= ry)
        {
            adj.entry(s).or_insert_with(Vec::new).push(n);
        }
    }

    println!("{:?}", adj);

    let sum: usize = adj
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();

    println!("{sum}");
}

// 530977 too high
// 527446
