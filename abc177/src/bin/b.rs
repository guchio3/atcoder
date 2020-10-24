use proconio::{input, marker::Chars};
use std::cmp::min;

fn main() {
    input!{
        s: Chars,
        t: Chars,
    }
    let mut res = t.len();

    for s_start in 0..=(s.len() - t.len()) {
        let mut temp_res = 0;
        for i in 0..t.len() {
            if s[s_start+i] != t[i] {
                temp_res += 1;
            }
        }
        res = min(res, temp_res);
    }

    println!("{}", res);
}
