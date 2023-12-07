mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn main() {
    match 7 {
        1 => d01::solve(),
        2 => d02::solve(),
        3 => d03::solve(),
        4 => d04::solve(),
        5 => d05::solve(),
        6 => d06::solve(),
        7 => d07::solve(),
        _ => unreachable!(),
    }
}
