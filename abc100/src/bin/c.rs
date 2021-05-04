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
        n: usize,
        a: [usize; n]
    }
    let mut res = 0;
    for mut a_i in a {
        while a_i % 2 == 0 {
            a_i /= 2;
            res += 1;
        }
    }
    println!("{}", res);
}
