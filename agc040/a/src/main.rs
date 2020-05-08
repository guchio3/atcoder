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
    let s: Vec<char> = read_line::<String>()[0].chars().collect();
    let mut a: Vec<i64> = vec![0; s.len() + 1];

    for i in 0..s.len() {
        if s[i] == '<' {
            a[i + 1] = a[i] + 1;
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] == '>' && a[i] <= a[i + 1] {
            a[i] = a[i + 1] + 1;
        }
    }

    println!("{}", a.iter().fold(0, |a, x| a + x));
}
