use std::io::stdin;
use std::str::FromStr;
use std::cmp::min;

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
    let in_vec: Vec<i64> = read_line();
    let n = in_vec[0];
    let k = in_vec[1];

    let tmp = n % k;
    let res = min(tmp, k - tmp);

    println!("{}", res);
}

