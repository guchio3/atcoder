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

fn read_str_as_T<T>() -> T 
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().unwrap()
}

fn main() {
    let k: usize = read_line()[0];
    let in_vec: Vec<usize> = read_line();
    let a = in_vec[0];
    let b = in_vec[1];

    for i in a..=b {
        if i % k == 0 {
            println!("OK");
            return;
        }
    }
    println!("NG");
}
