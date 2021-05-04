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
    let all_sum = a.iter().map(|x| *x).sum::<usize>();
    let mut tmp_sum = 0;
    let mut res = all_sum;
    for a_i in a {
        tmp_sum += a_i;
        let diff = all_sum - tmp_sum;
        res = min(res, max(diff, tmp_sum) - min(diff, tmp_sum));
    }
    println!("{}", res);
}
