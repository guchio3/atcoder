use std::io::stdin;
use std::str::FromStr;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let x: i64 = read_line()[0];

    let mut res_a: i64 = 0;
    let mut res_b: i64 = 0;
    for a in 0..120{
        for b in -120..120{
            if a < b {
                break;
            }
            if i64::pow(a, 5) - i64::pow(b, 5) == x {
                res_a = a;
                res_b = b;
            }
        }
    }

    println!("{} {}", res_a, res_b);
}
