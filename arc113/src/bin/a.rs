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
        k: usize
    }
    let mut res = 0;
    for a in 1..=k {
        let bc_lim = k / a + (k % a != 0) as usize;
        for b in 1..=bc_lim {
            let c_lim = k / (a * b);
            res += c_lim;
        }
    }
    println!("{}", res);
}
