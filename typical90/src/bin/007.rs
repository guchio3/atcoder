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
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q]
    }
    a.sort();
    let mut b_w_index = vec![];
    for i in 0..q {
        b_w_index.push((b[i], i));
    }
    b_w_index.sort();

    let mut reses = vec![1_000_000_000; q];
    let mut a_index = 0;
    for (b_i, b_i_index) in b_w_index {
        // ここを a_index < n - 1 && (b_i - a[a_index]).abs() > (b_i - a[a_index + 1]).abs()
        // とすると WA, なんで？？？？？？？？？？？？？？？？？？
        while a_index < n - 1 && (b_i - a[a_index]).abs() >= (b_i - a[a_index + 1]).abs() {
            a_index += 1;
        }
        reses[b_i_index] = (b_i - a[a_index]).abs();
    }

    println!("{}", reses.into_iter().join("\n"));
}
