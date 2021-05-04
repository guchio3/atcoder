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
    }
    let mut res = 0;
    for i in 1..=1_000_000 {
        let mut tmp_digit = 0;
        let mut i_copy = i;
        while i_copy > 0 {
            tmp_digit += 1;
            i_copy /= 10;
        }
        if i * 10usize.pow(tmp_digit) + i <= n {
            res += 1;
        }
    }
    println!("{}", res);
}
