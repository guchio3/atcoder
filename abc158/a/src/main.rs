use std::io::stdin;
use std::str::FromStr;

fn read_str_as_char_vec () -> Vec<char>
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().chars().collect()
}

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
    let s = read_str_as_char_vec();
    
    if s[0] == s[1] && s[1] == s[2] {
        println!("No");
    } else {
        println!("Yes");
    }
}
