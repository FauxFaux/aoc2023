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
            for y in 0..h - 1 {
                if (0..=y).all(|d| {
                    let o = d + 1;
                    y + o >= h || y < d || game[y - d] == game[y + o]
                }) {
                    return (0, y + 1);
                }
            }

            for x in 0..w - 1 {
                if (0..=x).all(|d| {
                    let o = d + 1;
                    x + o >= w || x < d || (0..h).all(|y| game[y][x - d] == game[y][x + o])
                }) {
                    return (x + 1, 0);
                }
            }

            return (0, 0);
        };

        let (x, y) = search(&game);
        if x == 0 && y == 0 {
            panic!("no solution");
        }
        if x == 0 {
            score += y * 100;
        } else {
            score += x;
        }
    }

    println!("{}", score);
}
