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
    let a = in_vec[0];
    let b = in_vec[1];

    let mut res = (b - 1) / (a - 1);
    if (b - 1) % (a - 1) != 0 {
        res += 1;
    }
    println!("{}", res);
}
