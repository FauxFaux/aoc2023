mod d01;
mod d02;

fn main() {
    match 2 {
        1 => d01::solve(),
        2 => d02::solve(),
        _ => unreachable!(),
    }
}
