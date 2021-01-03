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
        s: Chars
    }
    let mut cnt_dict = HashMap::new();
    cnt_dict.insert('a', 0);
    cnt_dict.insert('b', 0);
    cnt_dict.insert('c', 0);
    for s_i in s {
        *cnt_dict.get_mut(&s_i).unwrap() += 1;
    }
    let mut values: Vec<_> = cnt_dict.values().collect();
    values.sort();
    let first = values[2];
    let third = values[0];
    if first - third > 1 {
        println!("NO");
    } else {
        println!("YES");
    }
}
