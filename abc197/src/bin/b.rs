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
        h: usize, w: usize, x: usize, y: usize,
        s: [Chars; h]
    }
    let mut res = 0;
    let mut i = x;
    let j = y;
    while i > 0 && s[i - 1][j - 1] != '#' {
        res += 1;
        i -= 1
    }
    let mut i = x;
    let j = y;
    while i <= h && s[i - 1][j - 1] != '#' {
        res += 1;
        i += 1
    }
    let i = x;
    let mut j = y;
    while j > 0 && s[i - 1][j - 1] != '#' {
        res += 1;
        j -= 1
    }
    let i = x;
    let mut j = y;
    while j <= w && s[i - 1][j - 1] != '#' {
        res += 1;
        j += 1
    }
    println!("{}", res - 3);
}
