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
    let mut a_copy = a.clone();
    a_copy.sort_by(|x, y| y.partial_cmp(&x).unwrap());
    let largest = a_copy[0];
    let second_largest = a_copy[1];
    for a_i in a {
        if a_i == largest {
            println!("{}", second_largest);
        } else {
            println!("{}", largest);
        }
    }
}
