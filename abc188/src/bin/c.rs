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
        a: [usize; 2.pow(n as u32)]
    }
    let mut max_num = 0;
    let mut max_index = 0;
    for i in 0..a.len() {
        if a[i] > max_num {
            max_num = a[i];
            max_index = i;
        }
    }
    if max_index >= (a.len() / 2) {
        max_num = 0;
        max_index = 0;
        for i in 0..a.len() / 2 {
            if a[i] > max_num {
                max_num = a[i];
                max_index = i;
            }
        }
    } else {
        max_num = 0;
        max_index = 0;
        for i in a.len() / 2..a.len() {
            if a[i] > max_num {
                max_num = a[i];
                max_index = i;
            }
        }
    }
    println!("{}", max_index + 1);
}
