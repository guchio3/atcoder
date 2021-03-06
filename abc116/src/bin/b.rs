#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn f(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    input! {
        mut s: usize
    }
    let mut m_map = vec![false; 1000000];
    let mut res = 1;
    while !m_map[s] {
        m_map[s] = true;
        s = f(s);
        res += 1;
    }
    println!("{}", res);
}
