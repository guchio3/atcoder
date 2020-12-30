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
        mut a: [usize; n]
    }
    a.sort();
    let mut res = 0;
    let mut bef_a_i = 0;
    let mut forward_sum = a.iter().sum::<usize>();
    for a_i in a.into_iter().rev() {
        if forward_sum * 2 >= bef_a_i {
            res += 1;
            forward_sum -= a_i;
            bef_a_i = a_i;
        } else {
            break;
        }
    }
    println!("{}", res);
}
