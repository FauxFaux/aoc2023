use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Face {
    N,
    E,
    S,
    W,
}

impl Face {
    fn delta(&self) -> (i64, i64) {
        match self {
            Face::N => (0, -1),
            Face::E => (1, 0),
            Face::S => (0, 1),
            Face::W => (-1, 0),
        }
    }
}

pub fn solve() {
    let grid = include_str!("d16.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let h = grid.len() as i64;
    let w = grid[0].len() as i64;
    let max_wh = h.max(w);

    let co = |v: i64| {
        assert!(v < max_wh, "{v} < max_wh");
        usize::try_from(v).expect("co")
    };

    let mut visited = vec![vec![0i64; w as usize]; h as usize];
    visited[co(0)][co(0)] = 1;
    let mut beams = vec![(0, 0, Face::S)];
    while !beams.is_empty() {
        if false {
            for (x, y, f) in &beams {
                for sy in 0..h {
                    for sx in 0..w {
                        print!(
                            "{}",
                            if *x == sx && *y == sy {
                                match f {
                                    Face::N => '^',
                                    Face::E => '>',
                                    Face::S => 'v',
                                    Face::W => '<',
                                }
                            } else {
                                grid[co(sy)][co(sx)]
                            }
                        );
                    }
                    println!();
                }
                println!();
            }
        }
        let mut next = Vec::new();
        for (x, y, f) in beams {
            let (dx, dy) = f.delta();
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= w || ny < 0 || ny >= h {
                continue;
            }

            visited[co(ny)][co(nx)] += 1;

            let chr = grid[co(ny)][co(nx)];
            use Face::*;
            match chr {
                '.' => {
                    next.push((nx, ny, f));
                }
                '-' if f == E || f == W => {
                    next.push((nx, ny, f));
                }
                '|' if f == N || f == S => {
                    next.push((nx, ny, f));
                }
                '-' => {
                    next.push((nx, ny, E));
                    next.push((nx, ny, W));
                }
                '|' => {
                    next.push((nx, ny, N));
                    next.push((nx, ny, S));
                }
                '/' => {
                    next.push((
                        nx,
                        ny,
                        match f {
                            N => E,
                            E => N,
                            S => W,
                            W => S,
                        },
                    ));
                }
                '\\' => {
                    next.push((
                        nx,
                        ny,
                        match f {
                            N => W,
                            E => S,
                            S => E,
                            W => N,
                        },
                    ));
                }
                _ => unreachable!("{chr}"),
            }
        }

        println!("{next:?}");
        beams = next;

        for y in 0..h {
            for x in 0..w {
                print!("{}", if visited[co(y)][co(x)] > 0 { '#' } else { '.' });
            }
            println!();
        }

        let sum = visited
            .iter()
            .map(|r| r.iter().filter(|v| **v > 0).count())
            .sum::<usize>();
        println!("{sum}");
    }

    // 8110 too high
}
