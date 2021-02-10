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
        n: usize, m: usize,
        s: Chars,
        t: Chars
    }
    let lcm_num = lcm(n, m);
    let mut s_set = HashSet::new();
    let mut t_set = HashSet::new();
    for i in 0..n {
        s_set.insert((lcm_num / n) * i + 1);
    }
    for i in 0..m {
        t_set.insert((lcm_num / m) * i + 1);
    }
    for index in s_set.intersection(&t_set) {
        if s[(index - 1) / (lcm_num / n)] != t[(index - 1) / (lcm_num / m)] {
            println!("-1");
            return;
        }
    }
    println!("{}", lcm_num);
}
