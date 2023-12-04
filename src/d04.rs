use itertools::Itertools;

pub fn solve() {
    let m: u32 = include_str!("d04.txt")
        .lines()
        .map(|l| {
            let (_, gs) = l.split_once(':').expect("colon");
            let (w, h) = gs.split_once('|').expect("pipe");
            let w = w.split_whitespace().collect_vec();
            let h = h.split_whitespace().collect_vec();
            let c = h.iter().filter(|&h| w.contains(h)).count();
            if c == 0 {
                0
            } else {
                2u32.pow((c - 1) as u32)
            }
            // (w, h)
        })
        .sum();

    println!("{m:?}");
}
