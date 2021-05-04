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
        xy: [(usize, usize); m]
    }
    let mut nums = vec![1; n];
    let mut reds = vec![false; n];
    reds[0] = true;
    for (xi, yi) in xy {
        nums[xi - 1] -= 1;
        nums[yi - 1] += 1;
        if reds[xi - 1] {
            reds[yi - 1] = true;
        }
        if nums[xi - 1] == 0 {
            reds[xi - 1] = false;
        }
    }
    let mut res = 0;
    for red in reds {
        if red {
            res += 1;
        }
    }
    println!("{}", res);
}
