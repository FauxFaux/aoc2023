mod d01;
mod d02;
mod d03;

fn main() {
    match 3 {
        1 => d01::solve(),
        2 => d02::solve(),
        3 => d03::solve(),
        _ => unreachable!(),
    }
}
