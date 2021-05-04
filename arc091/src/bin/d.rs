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
    // (n / i) * (i - k) $B$N(B (i - k) $B$K3d$kJ}$,(B = 0 $B$N%1!<%9$b49;;$5$l$F$$$k(B * (1..=n)$B8D(B
    // $B$J$N$G!"(Bk = 0 $B$N%1!<%9$rH4$/I,MW$"$j(B
    if k == 0 {
        res -= n;
    }
    println!("{}", res);
}
