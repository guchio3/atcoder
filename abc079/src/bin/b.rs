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
    if n < 2 {
        println!("1");
    } else {
        let mut l0: usize = 2;
        let mut l1: usize = 1;
        let mut l2: usize = 0;
        for _i in 1..n {
            l2 = l0 + l1;
            l0 = l1;
            l1 = l2;
        }
        println!("{}", l2);
    }
}
