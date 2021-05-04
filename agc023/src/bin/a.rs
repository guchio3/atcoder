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
        a: [i64; n]
    }
    let mut cumsum = 0;
    let mut cumsums = vec![];
    for i in 0..n {
        cumsum += a[i];
        cumsums.push(cumsum);
    }

    let mut cnt_dict = HashMap::new();
    for i in 0..n {
        *(cnt_dict.entry(cumsums[i]).or_insert(0)) += 1;
    }

    let mut res: usize = 0;
    for i in 0..n {
        let cumsum = cumsums[i];
        let cum_minus_a = cumsums[i] - a[i];
        if let Some(cnt) = cnt_dict.get(&cum_minus_a) {
            res += cnt;
        }
        *cnt_dict.get_mut(&cumsum).unwrap() -= 1;
    }
    println!("{}", res);
}
