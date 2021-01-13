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
        x: i64,
        y: i64,
    }
    if x == y {
        println!("0");
    } else if y < x {
        println!("{}", x - y);
    } else {
        let mut res = 0;
        let mut i = 1;
        while x * 2.pow(i) < y {
            i += 1;
        }
        res += i;
        let diff = y - x.pow(i);
        let mut i = 1;
        while 2.pow(i) < diff {
            i += 1;
        }
    }
}
