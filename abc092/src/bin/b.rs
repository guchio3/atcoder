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
        d: usize, mut x: usize,
        a: [usize; n]
    }
    for a_i in a {
        let mut day = 1;
        while day <= d {
            x += 1;
            day += a_i;
        }
    }
    println!("{}", x);
}
