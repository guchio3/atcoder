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
    let _n: usize = read_line()[0];
    let a: Vec<i64> = read_line();

    let mut res: f64 = 1.;
    for a_i in a {
        if a_i == 0 {
            println!("0");
            return;
        }
        if res < 0. {
            continue;
        }
        let num = (1_000_000_000_000_000_000. / res) as i64;
        // # let num_mod = 1_000_000_000_000_000_000. % res;
        // # let ai_mod = 1_000_000_000_000_000_000. % a_i as f64;
        // # println!("{}, {}, {}, {}", (a_i == num), (num_mod > ai_mod), num_mod, ai_mod);
        // # println!("{}, {}, {}, {}, {}", res, a_i, num, num_mod, ai_mod);
        println!("{}, {}", res, num);
        // if (a_i > num) || ((a_i == num) && (num_mod > ai_mod)) {
        if (a_i > num) {
            res = -1.;
        } else {
            res = res * a_i as f64;
        }
    }
    if res < 0. {
        println!("-1");
    } else {
        println!("{}", res as usize);
    }
}
