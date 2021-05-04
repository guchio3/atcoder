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
        a: [i64; n],
        b: [i64; n]
    }
    let mut b_larger_num = 0;
    let mut a_larger_num = 0;
    for i in 0..n {
        let ai = a[i];
        let bi = b[i];
        if bi > ai {
            if (bi - ai) % 2 == 0 {
                b_larger_num += bi - ai;
            } else {
                b_larger_num += bi - ai - 1;
            }
        } else {
            a_larger_num += ai - bi;
        }
    }
    b_larger_num -= 2 * a_larger_num;
    if b_larger_num >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
