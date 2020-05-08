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

fn main() {
    let x: usize = read_line()[0];

    let mut res = 0;
    let mut base: usize = 100;
    while (base as usize) < x {
        base = ((base as f64) * 1.01) as usize;
        res += 1;
    }

    println!("{}", res);
    // let a_base = (x/100.).log(1.01);
    // if a_base.fract() == 0. {
    //     println!("{}", a_base as usize);
    // } else {
    //     println!("{}", a_base as usize + 1);
    // }
}
