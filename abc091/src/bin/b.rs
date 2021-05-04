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
        s: [String; n],
        m: usize,
        t: [String; m]
    }

    let mut cnt_map: HashMap<String, i64> = HashMap::new();
    for s_i in s {
        *cnt_map.entry(s_i).or_insert(0) += 1;
    }
    for t_i in t {
        *cnt_map.entry(t_i).or_insert(0) -= 1;
    }

    let mut res: i64 = 0;
    for (_key, value) in cnt_map {
        res = max(res, value);
    }

    println!("{}", res);
}
