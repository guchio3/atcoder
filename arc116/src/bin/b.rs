#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 998_244_352;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut res = 0;
    let mut min_num = 998_244_352 + 1;
    let mut min_diff = 0;
    let mut bef_min_diff = 0;
    let mut max_num = 0;
    let mut max_diff = 0;
    let mut bef_max_diff = 0;
    for i in 0..n {
        if a[0] > max_num {
            bef_max_diff = max_diff;
            max_diff = 0;
            max_num = a[0];
        }
        if a[0] < min_num {
            bef_min_diff = min_diff;
            min_diff = 0;
            min_num = a[0];
        }
        max_diff += 1;
        bef_max_diff += 1;
        min_diff += 1;
        bef_min_diff += 1;
        res = res + ;
    }
}
