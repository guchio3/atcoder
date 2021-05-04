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
        q: usize, h: usize, s: usize, d: usize,
        mut n: usize
    }
    n *= 100;
    let mut ice_teas = vec![(q * 8, 25, q), (h * 4, 50, h), (s * 2, 100, s), (d, 200, d)];
    ice_teas.sort();
    let mut ice_teas: VecDeque<(_, _, _)> = ice_teas.into_iter().collect();
    let mut res = 0;
    while ice_teas.len() > 0 {
        let ice_tea = ice_teas.pop_front().unwrap();
        res += ice_tea.2 * (n / ice_tea.1);
        n -= ice_tea.1 * (n / ice_tea.1);
    }
    println!("{}", res);
}
