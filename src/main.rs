mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    match 4 {
        1 => d01::solve(),
        2 => d02::solve(),
        3 => d03::solve(),
        4 => d04::solve(),
        _ => unreachable!(),
    }
}
