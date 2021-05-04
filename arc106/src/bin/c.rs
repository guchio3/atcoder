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
        n: i64, m: i64
    }
    if m == 0 {
        for i in 0..n {
            println!("{} {}", 1 + i, 1_000_000_000 - i);
        }
    } else if m > 0 {
        if n - m < 2 {
            println!("-1");
            return;
        }
        for i in 0..(n - m - 1) {
            println!("{} {}", 1 + i, 1_000_000_000 - i);
        }
        let mut l = n - m - 1 + 1;
        for _i in 0..m + 1 {
            println!("{} {}", l, l + 1);
            l += 2;
        }
    } else {
        println!("-1");
    }
}
