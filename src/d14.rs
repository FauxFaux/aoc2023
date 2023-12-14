use itertools::Itertools;

pub fn solve() {
    let mut grid = include_str!("d14.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    for _ in 0..h {
        for sy in 1..h {
            for x in 0..w {
                if grid[sy][x] != 'O' {
                    continue;
                }

                if grid[sy - 1][x] != '.' {
                    continue;
                }
                grid[sy - 1][x] = 'O';
                grid[sy][x] = '.';
            }

            // for y in 0..h {
            //     for x in 0..w {
            //         print!("{}", grid[y][x]);
            //     }
            //     println!();
            // }
            // println!();
        }
    }

    let load: usize = grid
        .iter()
        .enumerate()
        .map(|(y, r)| (h - y) * r.iter().filter(|c| **c == 'O').count())
        .sum();

    println!("{}", load);
}
