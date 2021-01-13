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
        mut a: [usize; n]
    }
    a.sort();
    let mut first = 0;
    let mut second = 0;
    let mut bef = 0;
    for a_i in a.into_iter().rev() {
        if a_i == bef {
            if first == 0 {
                first = a_i;
            } else {
                second = a_i;
                break;
            }
            bef = 0;
        } else {
            bef = a_i;
        }
    }
    println!("{}", first * second);
}
