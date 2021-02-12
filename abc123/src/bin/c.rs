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
        n: usize, a: usize, b: usize, c: usize, d: usize, e: usize
    }
    let mut min_num = 1_000_000_000_000_000;
    min_num = min(min_num, a);
    min_num = min(min_num, b);
    min_num = min(min_num, c);
    min_num = min(min_num, d);
    min_num = min(min_num, e);
    println!("{}", (n / min_num) + (n % min_num != 0) as usize + 4);
}
