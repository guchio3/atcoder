#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, m: usize,
        x: usize, y: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let mut cur_time = 0;
    let mut a_iter = 0;
    let mut b_iter = 0;
    let mut res = -1;
    while a_iter < n && b_iter < m {
        res += 1;
        while a_iter < n && a[a_iter] < cur_time {
            a_iter += 1;
        }
        if a_iter < n {
            cur_time = a[a_iter];
            cur_time += x;
        }
        while b_iter < m && b[b_iter] < cur_time {
            b_iter += 1;
        }
        if b_iter < m {
            cur_time = b[b_iter];
            cur_time += y;
        }
    }
    println!("{}", res);
}
