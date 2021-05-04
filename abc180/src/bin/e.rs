use proconio::{fastout, input};
use std::cmp::max;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n]
    }

    let mut costs = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let p_i = xyz[i];
            let p_j = xyz[j];
            costs[i][j] = (p_i.0 - p_j.0).abs() + (p_i.1 - p_j.1).abs() + max(0, p_j.2 - p_i.2);
        }
    }

    let mut bh = BinaryHeap::new();
    while bh.len() > 0 {
        ;
    }
}
