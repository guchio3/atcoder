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
    let n: usize = read_line()[0];
    let a: Vec<usize> = read_line();

    let mut res_vec: Vec<usize> = vec![0; n];
    
    for a_i in a {
        res_vec[a_i - 1] += 1;
    }

    for res_i in res_vec {
        println!("{}", res_i);
    }
}
