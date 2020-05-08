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
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let p = in_vec[1];

    let s = read_str_as_char_vec();

    if vec![2, 5].contains(&p) {
        let mut res = 0;
        for (i, &s_i) in s.iter().enumerate() {
            if (s_i as usize - '0' as usize) % p == 0 {
                res += i + 1;
            }
        }
        println!("{}", res);
        return;
    }

    let mut cnt_vec: Vec<usize> = vec![0; p as usize];
    cnt_vec[0] += 1;

    let mut bef_mod = 0;
    let mut bef_base_mod = 1 % p;
    for &s_i in s.iter().rev() {
        bef_mod = (bef_base_mod * (s_i as usize - '0' as usize) + bef_mod) % p;
        cnt_vec[bef_mod as usize] += 1;
        bef_base_mod = (bef_base_mod * 10) % p;
    }

    let mut res = 0;
    for cnt in cnt_vec {
        if cnt > 1 {
            res += cnt * (cnt - 1) / 2;
        }
    }

    println!("{}", res);
}
