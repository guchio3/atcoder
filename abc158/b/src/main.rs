use std::cmp::min;
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

fn read_str_as_char_vec() -> Vec<char> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().chars().collect()
}

fn main() {
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let a = in_vec[1];
    let b = in_vec[2];

    let mut res = n / (a + b) * a;

    let margin = n % (a + b);
    res += min(margin, a);

    println!("{}", res);
}
