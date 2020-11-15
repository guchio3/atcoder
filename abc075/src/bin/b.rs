use proconio::marker::Chars;
use proconio::{fastout, input};
use std::char;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mut ss = vec![vec!['.'; w + 2]; h + 2];
    for h_i in 0..h {
        for w_i in 0..w {
            ss[h_i + 1][w_i + 1] = s[h_i][w_i];
        }
    }
    let mut res = vec![vec!['#'; w]; h];
    for h_i in 0..h {
        for w_i in 0..w {
            if ss[h_i + 1][w_i + 1] == '#' {
                continue;
            } else {
                let mut cnt = 0;
                for i in 0..=2 {
                    for j in 0..=2 {
                        // cnt += (ss[h_i + 1 + i - 1][w_i + 1 + j - 1]) as usize;
                        cnt += (ss[h_i + i][w_i + j] == '#') as usize;
                    }
                }
                res[h_i][w_i] = char::from_digit(cnt as u32, 10).unwrap();
            }
        }
    }

    for h_i in 0..h {
        let res_i: String = res[h_i].iter().collect();
        println!("{}", res_i);
    }
}
