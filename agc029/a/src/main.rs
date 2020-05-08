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
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let s: Vec<char> = s.trim().chars().collect();

    let mut cum_cnt: usize = 0;
    let mut res: usize = 0;
    for s_i in s {
        if s_i == 'B' {
            cum_cnt += 1;
        } else if s_i == 'W' {
            res += cum_cnt;
        }
    }
    println!("{}", res);
}
