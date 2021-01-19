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
        n: usize, mut x: usize,
        mut a: [usize; n]
    }
    a.sort();
    let mut res = 0;
    let mut i = 0;
    for &a_i in a.iter() {
        if x >= a_i {
            res += 1;
            x -= a_i;
        } else {
            break;
        }
        i += 1;
    }
    if x != 0 && i == a.len() {
        res -= 1;
    }
    println!("{}", res);
}
