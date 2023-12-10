pub fn solve() {
    let lookup = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let r: u32 = include_str!("d01.txt")
        .lines()
        .map(|s| {
            let mut o = String::new();
            let mut i = 0;
            'p: loop {
                if i >= s.len() {
                    break;
                }
                for (n, d) in &lookup {
                    if s[i..].starts_with(n) {
                        o.push_str(&d.to_string());
                        i += n.len();
                        continue 'p;
                    }
                }
                o.push_str(&s[i..i + 1]);
                i += 1;
            }
            let lw = |s: &str| lookup.iter().filter_map(|(n, _)| s.rfind(n)).max();
            let ld = |s: &str| {
                lookup
                    .iter()
                    .filter_map(|(_n, _)| s.rfind(|c: char| c.is_ascii_digit()))
                    .max()
            };

            let digits = o.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            let f = digits.first().expect("non-empty");

            let lw = lw(s).unwrap_or(0);
            let ld = ld(s).unwrap_or(0);

            let l = if lw > ld {
                let c = &s[lw..];
                let d = lookup
                    .iter()
                    .filter_map(|(n, d)| c.starts_with(n).then(|| d))
                    .next()
                    .expect("must exist");
                format!("{d}")
            } else {
                s[ld..ld + 1].to_string()
            };

            println!("{s} -> {o} ({f} {l})");
            format!("{f}{l}").parse::<u32>().expect("parse")
        })
        .sum();
    println!("{r}");
}
