#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    // できるだけ取り除かないほうが良い
    // できるだけ重複させたほうが良い
    // ↓
    // 小さい方に合わせる形で重複をできるだけさせる方針
    let mut diff_num = max(n, m) - min(n, m);
    
    let target_vec;
    if n > m {
        target_vec = b;
    } else {
        target_vec = a;
    }
    let mut best_non_dup_num = min(n, m);
    for i in 0..max(n, m) {
        for j in diff_num - (max(n, m) - 1 - i)..diff_num {
            ;
        }
    }
    println!("{}", diff_num + best_non_dup_num);
}
