#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }
    let mut aa_i = 0;
    let mut bb_i = 1_000_000;
    for i in 0..n {
        let a_i = a[i];
        let b_i = b[i];
        aa_i = max(aa_i, a_i);
        bb_i = min(bb_i, b_i);
    }
    if aa_i > bb_i {
        println!("0");
    } else {
        println!("{}", bb_i - aa_i + 1);
    }
}
