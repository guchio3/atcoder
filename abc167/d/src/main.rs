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
    let in_vec: Vec<usize> = read_line();
    let mut in_n = in_vec[0];
    let k = in_vec[1];

    let a: Vec<usize> = read_line();
    let mut a_accessed: Vec<usize> = vec![0; in_n];
    let mut a_bef_cnt: Vec<usize> = vec![0; in_n];

    let mut n = 1;
    let mut bef_n = 1;
    let mut cnt = 0;
    for i in 0..=k {
        bef_n = n;
        if a_accessed[n - 1] == 1 {
            let cnt_diff = cnt - a_bef_cnt[n - 1];
            let left = k - i;
            let left_iter = left % cnt_diff;
            for _j in 0..left_iter {
                n = a[n - 1];
            }
            println!("{}", n);
            return;
        } else {
            a_accessed[n - 1] = 1;
            a_bef_cnt[n - 1] = cnt;
            n = a[n - 1];
            cnt += 1;
        }
    }
    println!("{}", bef_n);
}
