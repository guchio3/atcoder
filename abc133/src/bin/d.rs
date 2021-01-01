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
        a: [usize; n],
    }
    let all_water = a.iter().sum::<usize>();
    let mut cum_water = 0;
    for i in 0..(n - 1) / 2 {
        cum_water += 2 * a[2 * i + 1];
    }
    let mut reses = vec![];
    let mut bef_mount_water = all_water - cum_water;
    for a_i in a {
        reses.push(bef_mount_water);
        bef_mount_water = a_i * 2 - bef_mount_water;
    }
    println!("{}", reses.iter().join(" "));
}
