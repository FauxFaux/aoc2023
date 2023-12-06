use itertools::Itertools;

pub fn solve() {
    let (ts, rs) = include_str!("d06.txt").split_once('\n').expect("newline");
    let ts = ts
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().expect("t"))
        .collect_vec();
    let rs = rs
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().expect("d"))
        .collect_vec();
    let gs = ts.iter().zip(rs.iter()).collect_vec();
    // [(7, 9), (15, 40), (30, 200)]

    let mut score = 1;
    for (t, r) in &gs {
        let mut wins = 0;
        for c in 0..**t {
            let d = c * (*t - c);
            // println!("{} {} {}", c, t, d);
            if d > **r {
                wins += 1;
            }
        }

        println!("{} {} {}", t, r, wins);
        score *= wins;
    }

    println!("{score}");
    // d = h * (t - h)
    // d = ht - h^2
}
