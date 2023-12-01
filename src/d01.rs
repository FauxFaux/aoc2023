pub fn solve() {
    let r: u32 = include_str!("d01.txt")
        .lines()
        .map(|l| {
            let digits = l.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            let f = digits.first().expect("non-empty");
            let l = digits.last().expect("non-empty");
            format!("{f}{l}").parse::<u32>().expect("parse")
        })
        .sum();
    println!("{r}");
}
