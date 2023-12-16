use itertools::Itertools;

pub fn solve() {
    let parts = include_str!("d15.txt").trim().split(',').collect_vec();

    let mut run = 0i64;

    for part in parts {
        let mut current = 0u8;
        for b in part.bytes() {
            current = current.wrapping_add(b);
            current = current.wrapping_mul(17)
        }

        run += current as i64;
    }

    println!("{run}");
}
