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
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let k = in_vec[1];

    let mut res_vec: Vec<usize> = vec![0; n];

    for _i in 0..k {
        let _d: usize = read_line()[0];
        let a: Vec<usize> = read_line();
        for a_i in a {
            res_vec[a_i - 1] += 1;
        }
    }

    let mut res = 0;
    for res_i in res_vec {
        if res_i == 0 {
            res += 1;
        }
    }

    println!("{}", res);
}
