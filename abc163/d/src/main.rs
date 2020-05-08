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
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let k = in_vec[1];

    let mut res: usize = 0;
    for i in k..=n+1 {
        res = (res + ((n + 1 - i) * i + 1) % 1_000_000_007) % 1_000_000_007;
    }

    println!("{}", res);
}
