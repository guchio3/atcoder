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
        n: usize
    }
    let mut res = vec![1; n];
    for i in 1..=n {
        let mut iter_i = 2 * i;
        while iter_i <= n {
            if res[iter_i - 1] == res[i - 1] {
                res[iter_i - 1] = res[i - 1] + 1;
            } else {
            }
            iter_i += i;
        }
    }
    println!("{}", res.into_iter().join(" "));
}
