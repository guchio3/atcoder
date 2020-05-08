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
    let in_vec: Vec<i32> = read_line();
    let k = in_vec[0];

    let mut cnt = 0;
    let mut i: i64 = 0;
    let mut res: i64 = 0;
    'outer: while cnt < k {
        i += 1;
        // .window 使ったほうがスマートそう
        let num_chars: Vec<char> = i.to_string().chars().collect();
        let num_chars_cnt = num_chars.len();
        for j in 0..num_chars_cnt {
            if j != num_chars_cnt - 1 {
                if (num_chars[j] as i32 - num_chars[j+1] as i32).abs() > 1 {
                    continue 'outer;
                }
            }
        }
        cnt += 1;
        res = i;
    }

    println!("{}", res);
}
