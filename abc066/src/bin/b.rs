use proconio::marker::Chars;
use proconio::{fastout, input};

// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    'outer: for i in 0..(s.len() / 2) {
        let end_idx = s.len() / 2 - 1 - i;
        // println!("{}", end_idx);
        for j in 0..end_idx {
            if s[j] != s[end_idx + j] {
                continue 'outer;
            }
        }
        println!("{}", s.len() - (i + 1) * 2);
        // let res: String = s[0..end_idx].iter().collect();
        // println!("{}", res);
        return;
    }
}
