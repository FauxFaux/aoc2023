use itertools::Itertools;

pub fn solve() {
    let games = include_str!("d13.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.chars().map(|c| c == '#').collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let mut score = 0;
    'game: for game in games {
        let h = game.len();
        let w = game[0].len();

        for y in 1..h - 1 {
            for x in 0..w {
                print!("{}", if game[y][x] { '#' } else { '.' });
            }
            println!();
        }
        println!();

        let search = |game: &Vec<Vec<bool>>| {
            let mut hits = Vec::with_capacity(4);
            for y in 0..h - 1 {
                if (0..h).all(|d| {
                    let o = d + 1;
                    y + o >= h || y < d || game[y - d] == game[y + o]
                }) {
                    hits.push((0, y + 1));
                }
            }

            for x in 0..w - 1 {
                if (0..w).all(|d| {
                    let o = d + 1;
                    x + o >= w || x < d || (0..h).all(|y| game[y][x - d] == game[y][x + o])
                }) {
                    hits.push((x + 1, 0));
                }
            }

            hits
        };

        let orig = search(&game);
        assert_eq!(orig.len(), 1);
        let orig = orig[0];

        for fy in 0..h {
            for fx in 0..w {
                let mut game = game.clone();
                game[fy][fx] = !game[fy][fx];
                println!("{} {} {}", fx, fy, game[fy][fx]);
                for y in 1..h - 1 {
                    for x in 0..w {
                        print!("{}", if game[y][x] { '#' } else { '.' });
                    }
                    if y == fy {
                        print!("  <");
                    }
                    println!();
                }
                println!();
                let now = search(&game)
                    .into_iter()
                    .filter(|c| *c != orig)
                    .collect_vec();
                if now.is_empty() {
                    continue;
                }
                assert_eq!(now.len(), 1);
                let (x, y) = now[0];
                if x == 0 {
                    score += y * 100;
                    continue 'game;
                } else {
                    score += x;
                    continue 'game;
                }
            }
        }
        panic!("no solution");
    }

    // 26269 too low
    // 34529 too low
    // 37882 too low
    println!("{}", score);
}
