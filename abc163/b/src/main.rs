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
    let _m = in_vec[1];
    let a: Vec<usize> = read_line();
    let sum = a.iter().fold(0, |sum, x| sum + x);

    if sum > n {
        println!("{}", -1);
    } else {
        println!("{}", n - sum);
    }
}
