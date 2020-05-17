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

fn mod_combination(n: usize, k: usize, mod_base: usize) -> usize {
    ;
}

fn main() {
    const mod_base: usize = 998_244_353;
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];
    let k = in_vec[2];

    // 一旦 n_i この箱を作った後、
    // どの箱を pair にするかを重複有りで数え上げる -> 普通に ○ と |
    let mut res = 0;
    let mut cum_n_i = 0;
    for k_i in (0..=k).rev() {
        let mut n_i = n - k_i;
        cum_n_i = cum_n_i * (m - 1) % mod_base;
        for _k_i in 0..k_i {
            n_i = n_i * mod_combination(n_i - 1 + k_i, k_i) % mod_base;
        }
    }

    println!("{}", res);
}
