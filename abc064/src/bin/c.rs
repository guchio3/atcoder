use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut colors = vec![0; 9];
    for a_i in a {
        for i in 0..8 {
            if i * 400 <= a_i && a_i < (i + 1) * 400 {
                colors[i] += 1;
            }
        }
        if 3200 <= a_i {
            colors[8] += 1;
        }
    }

    let mut res_min = 0;
    let mut res_max = 0;
    for i in 0..8 {
        if colors[i] > 0 {
            res_min += 1;
            res_max += 1;
        }
    }
    for _ in 0..colors[8] {
        // res_max = min(res_max + 1, 8);
        res_max = res_max + 1;
    }

    res_min = max(1, res_min);

    println!("{} {}", res_min, res_max);
}
