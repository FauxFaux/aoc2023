use itertools::Itertools;

pub fn solve() {
    let games = include_str!("d04.txt")
        .lines()
        .map(|l| {
            let (_, gs) = l.split_once(':').expect("colon");
            let (w, h) = gs.split_once('|').expect("pipe");
            let w = w.split_whitespace().collect_vec();
            let h = h.split_whitespace().collect_vec();
            let c = h.iter().filter(|&h| w.contains(h)).count();
            c
        })
        .collect_vec();

    let mut copies = vec![1u32; games.len()];

    for (i, w) in games.iter().enumerate() {
        for j in 0..*w {
            copies[i + j + 1] += copies[i];
        }
    }

    println!("{copies:?} {:?}", copies.iter().sum::<u32>());
}
