use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }
    let mut res = 0;
    let mut streaming_num = 0;
    let mut bef_h_i = 0;
    for h_i in h {
        if h_i <= bef_h_i {
            streaming_num += 1;
        } else {
            res = max(res, streaming_num);
            streaming_num = 0;
        }
        bef_h_i = h_i;
    }
    res = max(res, streaming_num);
    println!("{}", res);
}
