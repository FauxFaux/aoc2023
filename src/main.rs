mod d01;

fn main() {
    match 1 {
        1 => d01::solve(),
        _ => unreachable!(),
    }
}
