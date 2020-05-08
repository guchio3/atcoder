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

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn gcd_three(a: usize, b: usize, c: usize) -> usize {
    let tmp = gcd(a, b);
    gcd(tmp, c)
}

fn main() {
    let in_vec: Vec<usize> = read_line();
    let k = in_vec[0];

    let mut res: i64 = 0;
    for a in 1..k+1 {
        for b in 1..k+1 {
            for c in 1..k+1 {
                res += gcd_three(a, b, c) as i64;
            }
        }
    }

    println!("{}", res);
}
