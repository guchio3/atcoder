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
        w: Chars
    }
    let mut cnt = HashMap::new();
    for w_i in w {
        *cnt.entry(w_i).or_insert(0) += 1;
    }
    for (_key, value) in cnt {
        if value % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
