use itertools::Itertools;

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
        let ags = vec![gs.clone()]
            .iter()
            .cycle()
            .take(5)
            .flatten()
            .copied()
            .collect_vec();
        let ans = vec![ns.clone()]
            .iter()
            .cycle()
            .take(5)
            .flatten()
            .copied()
            .collect_vec();
        // println!("{gs:?} {ags:?}");
        let s = place(&ags, &ans);
        println!("{}", s);
        score += s;
    }

    println!("{score}");
}

fn place(tpl: &[char], to_place: &[usize]) -> usize {
    if tpl.is_empty() {
        return if to_place.is_empty() { 1 } else { 0 };
    }

    let mut run = 0;
    if tpl[0] != '#' {
        run += place(&tpl[1..], to_place)
    };

    if to_place.is_empty() {
        return run;
    }

    let space_needed = to_place.iter().sum::<usize>() + to_place.len() - 1;

    if tpl.len() < space_needed {
        return run;
    }

    // println!("{tpl:?} {to_place:?}");
    if tpl.len() >= space_needed {
        // println!("out of space: {space_needed} {run}");
        let placing = to_place[0];
        let at_end = tpl.len() == placing;
        if tpl[..placing].iter().all(|c| *c != '.') && (at_end || tpl[placing] != '#') {
            if at_end {
                run += 1;
            } else {
                run += place(&tpl[placing + 1..], &to_place[1..]);
            }
        }
    }

    // println!("placed a {placing}: {run}");
    return run;
}
