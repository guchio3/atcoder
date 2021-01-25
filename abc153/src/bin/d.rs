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
        mut h: usize
    }
    let mut res: usize = 0;
    let mut mons_num: usize = 1;
    while h > 0 {
        res += mons_num;
        h /= 2;
        mons_num *= 2;
    }
    println!("{}", res);
}
