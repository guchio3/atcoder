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
        h: usize, w: usize
    }
    // 偶数偶数 or 奇数奇数
    if h == 1 || w == 1 {
        println!("1");
    } else {
        let res = ((w + 1) / 2) * ((h + 1) / 2) + (w / 2) * (h / 2);
        println!("{}", res);
    }
}
