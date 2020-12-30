#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

/// Least Common Multiple (最小公倍数)
fn my_lcm<T>(x: T, y: T) -> T
where
    T: Integer + Copy,
{
    x * y / gcd(x, y)
}

fn main() {
    input! {
        a: usize, b: usize, c: usize, d: usize
    }
    // b 以下の mod から a-1 以下の mod を引く
    // c + d - cd_lcm
    let cd_lcm = my_lcm(c, d);
    let res = (b / c + b / d - (a - 1) / c - (a - 1) / d) - (b / cd_lcm - (a - 1) / cd_lcm);
    println!("{}", (b - a + 1) - res);
}
