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

fn main() {
    let in_vec: Vec<usize> = read_line();
    let a = in_vec[0];
    let b = in_vec[1];

    // a が [8.0, 9.0)
    // b が [10.0, 11.0)
    // の区間を走査する

    let mut res = 0;

    let base_margin = (a as f64) / 0.08;
    let mut base = if base_margin.fract() == 0. {
        ((a as f64) / 0.08) as usize
    } else {
        ((a as f64) / 0.08) as usize + 1
    };

    while (base as f64 * 0.08) as usize == a {
        if (base as f64 * 0.1) as usize == b {
            res = base;
            break;
        }
        base += 1;
    }

    if res == 0 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
