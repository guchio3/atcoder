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
        p: [usize; n],
        q: [usize; n]
    }
    let mut a = 0;
    let mut b = 0;
    for (i, perms) in (1..=n).permutations(n).enumerate() {
        let perm_vec: Vec<usize> = perms.into_iter().collect();
        if p == perm_vec {
            a = i;
        }
        if q == perm_vec {
            b = i
        }
    }
    println!("{}", max(a, b) - min(a, b));

    // let mut res = 0;
    // for i in 1..=n {
    //     res += (p[n - i] - q[n - i]) * ((n - 1) as i64).pow((i - 1) as u32);
    // }
    // println!("{}", res.abs());

    // let mut p_num = 0;
    // let mut q_num = 0;
    // for i in 1..=n {
    //     p_num += p[n - i] * 10.pow((i - 1) as u32);
    //     q_num += q[n - i] * 10.pow((i - 1) as u32);
    // }
    // println!("{}", p_num);
    // println!("{}", q_num);
    // println!("{}", max(p_num, q_num) - min(p_num, q_num));
}
