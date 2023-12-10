use itertools::Itertools;

pub fn solve() {
    let reads = include_str!("d09.txt")
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().expect("num"))
                .collect_vec()
        })
        .collect::<Vec<_>>();

    let mut stacks = Vec::new();

    for read in reads {
        let mut here = read.clone();
        let mut stack = Vec::with_capacity(8);
        stack.push(here.clone());
        loop {
            let d = here
                .iter()
                .zip(here.iter().skip(1))
                .map(|(a, b)| (b - a))
                .collect_vec();
            if d.iter().all(|d| *d == 0) {
                break;
            }
            stack.push(d.clone());
            here = d;
        }
        stacks.push(stack);
    }

    let sum = stacks
        .iter()
        .map(|stack| {
            let inits = stack
                .iter()
                .rev()
                .map(|s| s.first().expect("non-empty"))
                .copied()
                .collect_vec();
            let mut prev = 0;
            for i in inits {
                prev = i - prev;
            }
            prev
        })
        .sum::<i64>();

    println!("{:?}", sum);
}
