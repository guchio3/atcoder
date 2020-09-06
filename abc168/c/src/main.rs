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
    // 余弦定理
    let in_vec: Vec<usize> = read_line();
    let a = in_vec[0];
    let b = in_vec[1];
    let h = in_vec[2];
    let m = in_vec[3];

    let mut a_theta = (h as f64 + (m as f64) / 60.) / 12.;
    // if a_theta > 1. {
    //     a_theta -= 1.;
    // }
    let mut b_theta = (m as f64) / 60.;
    if b_theta > a_theta {
        let temp = b_theta;
        b_theta = a_theta;
        a_theta = temp;
    }
    let mut diff = a_theta - b_theta;
    if diff > 0.5 {
        diff = 1. - diff;
    }

    let res_pow = f64::powi(a as f64, 2) + f64::powi(b as f64, 2) - 2. * (a as f64) * (b as f64) * (diff * 2.0 * std::f64::consts::PI).cos();
    let res = f64::powf(res_pow, 0.5);
    println!("{}", res);
}
