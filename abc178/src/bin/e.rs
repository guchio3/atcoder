use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    // 45 度回転
    let mut xx = vec![];
    let mut yy = vec![];
    for (x_i, y_i) in xy {
        xx.push(x_i - y_i);
        yy.push(x_i + y_i);
    }
    xx.sort();
    yy.sort();
    println!(
        "{}",
        max(
            (xx[0] - xx[xx.len() - 1]).abs(),
            (yy[0] - yy[yy.len() - 1]).abs()
        )
    );
}
