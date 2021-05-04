#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut res = 0;
    let mut likes = vec![0; n];
    for i in 0..n {
        likes[i] = a[i];
        if likes[a[i] - 1] == i + 1 {
            res += 1;
        }
    }
    println!("{}", res);
}
