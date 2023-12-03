use itertools::Itertools;

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
            .filter_map(|(p, c)| b"&*#%$-@=+/".contains(c).then_some(p))
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
        let found = syms
            .iter()
            .find(|(x, y)| *x >= lx && *x <= rx && *y >= ly && *y <= ry);

        println!("{n} at {xs:?} {y} {found:?}");

        if found.is_some() {
            sum += n;
        }
    }

    // println!("{nums:?} {syms:?}");

    println!("{sum}");
}

// 530977 too high
// 527446
