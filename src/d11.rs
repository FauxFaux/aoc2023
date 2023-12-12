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

    let mut coords = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] {
                coords.push((x, y));
            }
        }
    }

    let g = 1000000 - 1;

    let mut ng = coords.clone();
    for gy in nr.iter().rev() {
        for ((_, oy), (_, ny)) in coords.iter().zip(ng.iter_mut()) {
            if oy > gy {
                *ny += g;
            }
        }
    }

    for gx in nc.iter().rev() {
        for ((ox, _), (nx, _)) in coords.iter().zip(ng.iter_mut()) {
            if ox > gx {
                *nx += g;
            }
        }
    }

    println!("{coords:?}");
    println!("{ng:?}");

    let coords = ng;

    let mut dists = Vec::new();
    let i6 = |v: usize| v as i64;
    let d = |a: usize, b: usize| (i6(a) - i6(b)).abs();

    let combs: Vec<((_, _), (_, _))> = coords.iter().copied().tuple_combinations().collect_vec();
    for ((ax, ay), (bx, by)) in combs {
        dists.push(d(ax, bx) + d(ay, by));
    }

    let sum = dists.iter().copied().sum::<i64>();

    println!("{sum}")
}
