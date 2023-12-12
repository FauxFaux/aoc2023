use itertools::{repeat_n, Itertools};

pub fn solve() {
    let games = include_str!("d12.txt")
        .lines()
        .map(|l| {
            let (g, n) = l.split_once(' ').expect("line");
            (
                g.chars().collect_vec(),
                n.split(',')
                    .map(|n| n.parse::<usize>().expect("num"))
                    .collect_vec(),
            )
        })
        .collect_vec();

    // gs: ?, . or #

    let mut score = 0usize;

    for (gs, ns) in games {
        let blanks = gs
            .iter()
            .enumerate()
            .filter_map(|(p, c)| (*c == '?').then_some(p))
            .collect_vec();
        let mut base = gs.iter().map(|c| *c == '#').collect_vec();

        for repl in repeat_n([false, true], blanks.len()).multi_cartesian_product()
        // .into_iter()
        // .permutations(blanks.len())
        {
            // println!("{repl:?}");
            for (r, p) in repl.into_iter().zip(blanks.iter().copied()) {
                base[p] = r;
            }

            let hits = base
                .iter()
                .group_by(|b| **b)
                .into_iter()
                .filter_map(|(b, l)| b.then_some(l.count()))
                .collect_vec();
            if hits == ns {
                score += 1;
            }

            let s = base
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>();
            // println!("{s}")
        }
    }

    println!("{score}");
}
