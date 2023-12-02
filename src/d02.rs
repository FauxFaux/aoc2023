use itertools::Itertools;

pub fn solve() {
    let r = include_str!("d02.txt")
        .lines()
        .map(|s| {
            let (game, moves) = s.split_once(":").expect("colon");
            let (_, game) = game.split_once(" ").expect("space");
            let game = game.parse::<u32>().expect("game");
            let moves = moves
                .trim()
                .split(';')
                .map(|m| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    for mv in m.split(',') {
                        let (n, colour) = mv.trim().split_once(' ').expect("space");
                        let n = n.parse::<u32>().expect("number");
                        match colour {
                            "red" => r += n,
                            "green" => g += n,
                            "blue" => b += n,
                            other => unreachable!("colour={other}"),
                        }
                    }
                    (r, g, b)
                })
                .collect_vec();
            let r = moves
                .iter()
                .copied()
                .map(|(r, _, _)| r)
                .max()
                .expect("moves");
            let g = moves
                .iter()
                .copied()
                .map(|(_, g, _)| g)
                .max()
                .expect("moves");
            let b = moves
                .iter()
                .copied()
                .map(|(_, _, b)| b)
                .max()
                .expect("moves");
            if r <= 12 && g <= 13 && b <= 14 {
                println!(
                    "X game={game}, moves={moves:?} {r} {g} {b}",
                    game = game,
                    moves = moves
                );
            } else {
                println!("0 game={game}, moves={moves:?}", game = game, moves = moves);
            }
            r*g*b
        })
        .sum::<u32>();
    println!("r={r}");
}
