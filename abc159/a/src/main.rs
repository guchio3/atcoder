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
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];

    if n <= 1 {
        if m <= 1 {
            println!("0");
        } else {
            println!("{}", m * (m - 1) / 2);
        }
    } else if m <= 1 {
        println!("{}", n * (n - 1) / 2);
    } else {
        println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2);
    }
}
