mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;

fn main() {
    match 14 {
        1 => d01::solve(),
        2 => d02::solve(),
        3 => d03::solve(),
        4 => d04::solve(),
        5 => d05::solve(),
        6 => d06::solve(),
        7 => d07::solve(),
        8 => d08::solve(),
        9 => d09::solve(),
        10 => d10::solve(),
        11 => d11::solve(),
        12 => d12::solve(),
        13 => d13::solve(),
        14 => d14::solve(),
        _ => unreachable!(),
    }
}
