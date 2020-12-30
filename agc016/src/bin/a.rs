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
        s: Chars,
    }
    let elements: HashSet<char> = s.iter().map(|&x| x).collect();
    
    let mut min_diff = 1000000;
    for element in elements {
        let mut cnt = 0;
        let mut max_diff = 0;
        for &s_i in s.iter() {
            if s_i == element {
                max_diff = max(max_diff, cnt);
                cnt = 0;
            } else {
                cnt += 1;
            }
        }
        max_diff = max(max_diff, cnt);
        min_diff = min(min_diff, max_diff);
    }
    println!("{}", min_diff);
}
