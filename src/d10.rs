use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Start,
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Compass {
    N,
    E,
    S,
    W,
}

impl Compass {
    fn off(self) -> (i64, i64) {
        use Compass::*;
        match self {
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
            W => (-1, 0),
        }
    }

    fn all() -> [Compass; 4] {
        use Compass::*;
        [N, E, S, W]
    }

    fn flip(self) -> Compass {
        use Compass::*;
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
}

impl Dir {
    fn regular(&self) -> bool {
        use Dir::*;
        matches!(self, NS | WE | NE | NW | SW | SE)
    }

    fn all_regular() -> [Dir; 6] {
        use Dir::*;
        [NS, WE, NE, NW, SW, SE]
    }

    fn has(&self, compass: Compass) -> bool {
        use Compass::*;
        use Dir::*;
        match compass {
            N => matches!(self, NW | NE | NS),
            E => matches!(self, NE | SE | WE),
            S => matches!(self, SW | SE | NS),
            W => matches!(self, NW | SW | WE),
        }
    }

    fn has_all(&self, compasses: &[Compass]) -> bool {
        compasses.iter().all(|&c| self.has(c))
    }

    fn adj(&self) -> [Compass; 2] {
        use Compass::*;
        use Dir::*;
        match self {
            NS => [N, S],
            WE => [W, E],
            NE => [N, E],
            NW => [N, W],
            SW => [S, W],
            SE => [S, E],
            other => unreachable!("{other:?}"),
        }
    }
}

pub fn solve() {
    use Dir::*;
    let grid = include_str!("d10.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => G,
                    '|' => NS,
                    '-' => WE,
                    'L' => NE,
                    'J' => NW,
                    '7' => SW,
                    'F' => SE,
                    'S' => Start,
                    other => unreachable!("non-char {other:?}"),
                })
                .collect_vec()
        })
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    let (sx, sy) = find(&grid, Start).expect("s");

    assert!(!Dir::G.has(Compass::N));

    let valid_starts = Compass::all()
        .into_iter()
        .filter(|c| {
            let (dx, dy) = c.off();
            let here = get(&grid, (sx + dx, sy + dy));
            let has = here.has(c.flip());
            println!("here: {:?} {:?} {}", here, c, has);
            has
        })
        .collect_vec();

    let start_can_be = Dir::all_regular()
        .into_iter()
        .filter(|d| d.has_all(&valid_starts))
        .collect_vec();
    println!("start_can_be: {start_can_be:#?}");

    assert_eq!(start_can_be.len(), 1);
    let start_tile = start_can_be[0];

    let (mut hx, mut hy) = (sx, sy);
    let mut dists = vec![vec![0; w]; h];
    let mut dist = 0;
    let (mut px, mut py) = (-1, -1);
    loop {
        let here = get(&grid, (hx, hy));
        let here = match here {
            Start => start_tile,
            G => unreachable!("walked into ground"),
            other => other,
        };
        let hd = &mut dists[uz(hy)][uz(hx)];
        if *hd != 0 {
            break;
        }
        *hd = dist;
        dist += 1;
        let mv = here
            .adj()
            .into_iter()
            .filter(|c| {
                let (dx, dy) = c.off();
                let (nx, ny) = (hx + dx, hy + dy);
                !(nx == px && ny == py)
            })
            .next()
            .expect("haven't visited somewhere");
        println!("{:?}: moving {:?} from {:?}", (hx, hy), mv, here);
        let (dx, dy) = mv.off();
        px = hx;
        py = hy;
        hx += dx;
        hy += dy;
    }

    println!("grid: {grid:#?}, dists: {dists:?}, dist: {dist}");
    println!("{}", (dist - 1) / 2);
}

fn uz(v: i64) -> usize {
    usize::try_from(v).expect("usize")
}

fn get(grid: &[Vec<Dir>], (x, y): (i64, i64)) -> Dir {
    if x < 0 || y < 0 {
        return Dir::G;
    }
    let y = y as usize;
    let x = x as usize;
    if y >= grid.len() {
        return Dir::G;
    }
    if x >= grid[y].len() {
        return Dir::G;
    }
    grid[y][x]
}

fn find(grid: &[Vec<Dir>], d: Dir) -> Option<(i64, i64)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == d {
                return Some((x as i64, y as i64));
            }
        }
    }
    None
}
