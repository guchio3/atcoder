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
    const TARGET: usize = 2;

    let n: usize = read_line()[0];
    let mut a: Vec<usize> = Vec::new();

    a.push(0);
    for _i in 0..n {
        a.push(read_line()[0]);
    }

    let mut res = 0;
    let mut next = 1;
    while res < n + 1 {
        res += 1;
        next = a[next];
        if next == TARGET {
            break;
        }
    }

    if res >= n {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
