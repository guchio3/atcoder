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
    let mut res_deque = VecDeque::new();
    for i in 0..n {
        let a_i = a[i];
        if (n % 2 != 0) ^ (i % 2 == 0) {
            res_deque.push_back(a_i);
        } else {
            res_deque.push_front(a_i);
        }
    }
    let res_vec: Vec<_> = res_deque.iter().collect();
    let res: String = res_vec.iter().join(" ");
    println!("{}", res);
}

