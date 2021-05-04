#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, k: usize
    }
    let mut res: usize = 0;
    for i in k + 1..=n {
        res += (n / i) * (i - k);
        if n % i >= k {
            res += n % i - k + 1;
        }
    }
    // (n / i) * (i - k) の (i - k) に割る方が = 0 のケースも換算されている * (1..=n)個
    // なので、k = 0 のケースを抜く必要あり
    if k == 0 {
        res -= n;
    }
    println!("{}", res);
}
