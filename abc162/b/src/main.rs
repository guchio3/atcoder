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

    let mut res: i64 = 0;
    for i in 1..n+1 {
        if (i % 3 != 0) && (i % 5 != 0) {
            res += i as i64;
        }
    }

    println!("{}", res);
}
