use itertools::Itertools;

pub fn solve() {
    let grid = include_str!("d11.txt")
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    let nr = (0..h)
        .filter(|y| grid[*y].iter().all(|v| *v == false))
        .collect_vec();
    let nc = (0..w)
        .filter(|x| (0..h).all(|y| !grid[y][*x]))
        .collect_vec();

    let mut ng = grid.clone();
    for y in nr.iter().rev() {
        ng.insert(*y, vec![false; w]);
    }

    let h = ng.len();

    println!("new rows:");
    for y in 0..h {
        for x in 0..w {
            print!("{}", if ng[y][x] { "#" } else { "." });
        }
        println!();
    }

    for x in nc.iter().rev() {
        for y in 0..h {
            ng[y].insert(*x, false);
        }
    }

    let w = ng[0].len();

    println!("new cols:");
    for y in 0..h {
        for x in 0..w {
            print!("{}", if ng[y][x] { "#" } else { "." });
        }
        println!();
    }

    let grid = ng;

    let mut coords = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] {
                coords.push((x, y));
            }
        }
    }

    let mut dists = Vec::new();
    let i6 = |v: usize| v as i64;
    let d = |a: usize, b: usize| (i6(a) - i6(b)).abs();
    // for (ax, ay) in &coords {
    //     for (bx, by) in &coords {
    //         if ax == bx && ay == by {
    //             continue;
    //         }
    //         dists.push(d(*ax, *bx) + d(*ay, *by) - 6);
    //     }
    // }

    let combs: Vec<((_, _), (_, _))> = coords.iter().copied().tuple_combinations().collect_vec();
    for ((ax, ay), (bx, by)) in combs {
        dists.push(d(ax, bx) + d(ay, by));
    }

    let sum = dists.iter().copied().sum::<i64>();

    println!("{sum}")
}
