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
        x: usize
    }
    let mut res = 1;
    for number in 2..x {
        let mut iter_num = number * number;
        while iter_num <= x {
            res = max(res, iter_num);
            iter_num *= number;
        }
    }
    println!("{}", res);
}
