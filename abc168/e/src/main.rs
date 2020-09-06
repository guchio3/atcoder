use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

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
    let n: usize = read_line()[0];
    let mut left_nums: HashMap<String, Vec<usize>> = HashMap::new();
    let mut right_nums: HashMap<String, Vec<usize>> = HashMap::new();

    for i in 0..n {
        let in_vec: Vec<i64> = read_line();
        let a = in_vec[0] as f64;
        let b = in_vec[1] as f64;
        if b != 0. {
            let left = a / b;
            (*left_nums.entry(left.to_string()).or_insert(vec![])).push(i);
        }
        if a != 0. {
            let right = - b / a;
            (*right_nums.entry(right.to_string()).or_insert(vec![])).push(i);
        }
    }
}
