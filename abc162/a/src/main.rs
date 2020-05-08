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
    let in_vec: Vec<i32> = read_line();
    let n = in_vec[0];
    let res: Vec<char> = n.to_string().chars().collect();

    if res.contains(&'7') {
        println!("Yes");
    } else {
        println!("No");
    }
}
