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
    }
    let mut res = 1;
    let mut res_cnt = 0;
    for i in 1..=n {
        let mut div_cnt = 0;
        let mut num = i;
        while num > 1 {
            if num % 2 == 0 {
                div_cnt += 1;
                num /= 2;
            } else {
                break;
            }
        }
        if div_cnt > res_cnt {
            res_cnt = div_cnt;
            res = i;
        }
    }
    println!("{}", res);
}
