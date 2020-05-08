use std::collections::VecDeque;
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

fn read_str_as_char_vec() -> VecDeque<char> {
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
    let mut s = read_str_as_char_vec();
    let q: usize = read_str_as_T();

    let mut rev = 1;
    for _i in 0..q {
        // let in_vec: Vec<char> = read_line();
        let mut temp = String::new();
        stdin().read_line(&mut temp).ok();
        let mut in_iter = temp.trim().split_whitespace();
        let mut t = in_iter.next().unwrap().chars();

        if t.next().unwrap() == '1' {
            rev *= -1;
        } else {
            let f: usize = in_iter.next().unwrap().parse().unwrap();
            let mut p = in_iter.next().unwrap().chars();
            if f == 1 {
                if rev == 1 {
                    s.push_front(p.next().unwrap());
                } else {
                    s.push_back(p.next().unwrap());
                }
            } else {
                if rev == -1 {
                    s.push_front(p.next().unwrap());
                } else {
                    s.push_back(p.next().unwrap());
                }
            }
        }
    }

    let mut res: String = String::new();
    if rev == 1 {
        for &c in s.iter() {
            res.push(c);
        }
    } else {
        for &c in s.iter().rev() {
            res.push(c);
        }
    }

    println!("{}", res);
}
