use itertools::Itertools;
use std::collections::HashMap;

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
        let mut meme = HashMap::with_capacity(10_000);
        let s = place(&ags, &ans, &mut meme);
        println!("{} {gs:?}", s);
        // break;
        score += s;
    }

    println!("{score}");
}

fn place(tpl: &[char], to_place: &[usize], meme: &mut HashMap<(usize, usize), usize>) -> usize {
    if tpl.is_empty() {
        return if to_place.is_empty() { 1 } else { 0 };
    }

    let key = (tpl.len(), to_place.len());
    if let Some(existing) = meme.get(&key) {
        return *existing;
    }

    let mut run = 0;
    if tpl[0] != '#' {
        run += place(&tpl[1..], to_place, meme)
    };

    if to_place.is_empty() {
        return run;
    }

    let space_needed = to_place.iter().sum::<usize>() + to_place.len() - 1;

    if tpl.len() < space_needed {
        meme.insert(key, run);
        return run;
    }

    // println!("{} {to_place:?}", tpl.iter().collect::<String>());

    let placing = to_place[0];
    let at_end = tpl.len() == placing;
    let is_match = tpl[..placing].iter().all(|c| *c != '.') && (at_end || tpl[placing] != '#');
    if !is_match {
        meme.insert(key, run);
        return run;
    }

    if at_end {
        run += place(&[], &to_place[1..], meme);
    } else {
        run += place(&tpl[placing + 1..], &to_place[1..], meme);
    }

    // println!("placed a {placing}: {run}");
    meme.insert(key, run);
    return run;
}
