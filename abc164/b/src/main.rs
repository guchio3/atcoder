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
    let a = in_vec[0];
    let b = in_vec[1];
    let c = in_vec[2];
    let d = in_vec[3];

    let t_score = c / b + (c % b != 0) as usize;
    let a_score = a / d + (a % d != 0) as usize;

    if t_score <= a_score {
        println!("Yes");
    } else {
        println!("No");
    }
    // loop {
    //     c -= b;
    //     if c <= 0 {
    //         println!("Yes");
    //         break;
    //     }

    //     a -= d;
    //     if a <= 0 {
    //         println!("No");
    //         break;
    //     }
    // }
}
