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
        n: usize, l: usize,
        a: [usize; n]
    }

    let mut res = 0;
    for a_i in a {
        if a_i >= l {
            res += 1
        }
    }
    println!("{}", res);
}
