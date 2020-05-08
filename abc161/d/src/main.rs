use std::io::stdin;
use std::str::FromStr;
use std::collections::VecDeque;

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

    let mut res: i64 = 0;

    let mut queue: VecDeque<i64> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i);
    }
    for _i in 0..k {
        let front_num = queue.pop_front().unwrap();
        res = front_num;
        let smallest_digit = front_num % 10;
        if smallest_digit != 0 {
            queue.push_back(front_num * 10 + smallest_digit - 1);
        }
        queue.push_back(front_num * 10 + smallest_digit);
        if smallest_digit != 9 {
            queue.push_back(front_num * 10 + smallest_digit + 1);
        }
    }

    println!("{}", res);
}
