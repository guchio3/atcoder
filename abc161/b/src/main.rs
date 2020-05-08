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
    let vars: Vec<i32> = read_line();
    // let n = vars[0];
    let m = vars[1];
    let a: Vec<i32> = read_line();

    let sum_a_i: f32 = a.iter().fold(0., |acc, &x| acc + x as f32);

    let mut cnt = 0;
    for a_i in a.iter() {
        if *a_i as f32 >= sum_a_i / (4. * m as f32) {
            cnt += 1;
        }
        if cnt >= m {
            break;
        }
    }

    if cnt >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
