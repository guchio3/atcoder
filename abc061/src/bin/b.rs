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
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }
    let mut cnts = vec![0; n];
    for (ai, bi) in ab {
        cnts[ai - 1] += 1;
        cnts[bi - 1] += 1;
    }
    for cnt in cnts {
        println!("{}", cnt);
    }
}
