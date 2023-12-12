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
        let mut ags = Vec::new();
        for _ in 0..5 {
            ags.extend_from_slice(&gs);
            ags.push('?');
        }
        ags.pop();
        let ans = vec![ns.clone()]
            .iter()
            .cycle()
            .take(5)
            .flatten()
            .copied()
            .collect_vec();
        // println!("{gs:?} {ags:?}");
        let s = place(&ags, &ans);
        println!("{} {gs:?}", s);
        // break;
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

    // println!("{} {to_place:?}", tpl.iter().collect::<String>());

    let placing = to_place[0];
    let at_end = tpl.len() == placing;
    let is_match = tpl[..placing].iter().all(|c| *c != '.') && (at_end || tpl[placing] != '#');
    if !is_match {
        return run;
    }

    if at_end {
        run += place(&[], &to_place[1..]);
    } else {
        run += place(&tpl[placing + 1..], &to_place[1..]);
    }

    // println!("placed a {placing}: {run}");
    return run;
}
