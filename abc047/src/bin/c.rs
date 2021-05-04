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
        s: Chars
    }
    let mut res_plus_1 = 0;
    let mut bef = 'a';
    for s_i in s {
        if s_i != bef {
            res_plus_1 += 1;
            bef = s_i;
        }
    }
    println!("{}", res_plus_1 - 1);
}
