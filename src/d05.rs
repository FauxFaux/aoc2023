use itertools::Itertools;

pub fn solve() {
    let mut lines = include_str!("d05.txt").lines();
    let pairs = lines
        .next()
        .expect("non-empty")
        .split_once(':')
        .expect("seeds colon")
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("seed number"))
        .tuples()
        .collect::<Vec<(i64, i64)>>();

    let mut seeds = Vec::with_capacity(99999);
    for (l, d) in pairs {
        for i in l..l + d {
            seeds.push(i);
        }
    }

    let mut cats = Vec::new();
    let mut cat = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("map:") {
            cats.push(cat);
            cat = Vec::new();
            continue;
        }

        let mut ns = line
            .split_whitespace()
            .map(|c| c.parse::<i64>().expect("num"));
        cat.push((
            ns.next().expect("1"),
            ns.next().expect("2"),
            ns.next().expect("3"),
        ));

        assert!(ns.next().is_none());
    }

    cats.push(cat);
    cats.remove(0);

    // let mut locs = Vec::new();

    let mut min = i64::MAX;

    for (n, seed) in seeds.iter().enumerate() {
        if n % 1_000_000 == 0 {
            println!("{}/{}", n, seeds.len());
        }
        // let mut path = Vec::new();
        let mut searching = *seed;
        for cat in &cats {
            let mut found = None;
            for (d, s, w) in cat {
                if searching >= *s && searching < s + w {
                    found = Some(searching - s + d);
                    break;
                }
            }
            searching = found.unwrap_or(searching);
            // path.push(searching);
            // println!("step at: {path:?}");
        }

        // println!("{seed} -> {path:?}");
        // let loc = *path.last().expect("non-empty");
        let loc = searching;
        if loc < min {
            min = loc;
        }
        // locs.push(loc)
    }

    // println!("{locs:?} {}", locs.iter().min().expect("non-empty"));

    println!("{min}");
}
