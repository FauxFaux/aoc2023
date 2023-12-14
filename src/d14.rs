use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let mut grid = include_str!("d14.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    let mut grids = HashMap::new();

    for i in 0..1000000000 {
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            loop {
                let mut changed = false;
                for sy in 0..h {
                    for sx in 0..w {
                        if grid[sy][sx] != 'O' {
                            continue;
                        }

                        let nx = sx as i64 + dx;
                        let ny = sy as i64 + dy;
                        if nx < 0 || nx >= w as i64 || ny < 0 || ny >= h as i64 {
                            continue;
                        }

                        let nx = nx as usize;
                        let ny = ny as usize;

                        if grid[ny][nx] != '.' {
                            continue;
                        }
                        grid[ny][nx] = 'O';
                        grid[sy][sx] = '.';
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        grids
            .entry(grid.clone())
            .or_insert_with(|| Vec::new())
            .push(i + 1);

        if i % 800 == 799 {
            println!("{}", i as f64 / 1000000000.0 * 100.0);
            for (k, v) in &grids {
                println!(
                    "s: {}, l: {:?}",
                    k.iter()
                        .enumerate()
                        .map(|(y, r)| (h - y) * r.iter().filter(|c| **c == 'O').count())
                        .sum::<usize>(),
                    v
                );
            }
            return;
        }
        // for y in 0..h {
        //     for x in 0..w {
        //         print!("{}", grid[y][x]);
        //     }
        //     println!();
        // }
        // println!();
    }

    // (1000000000-124)%(152-126)+124
    // grep 124
}
