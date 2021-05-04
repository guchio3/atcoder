#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        r: i64, x: i64, y: i64
    }
    let pow_r = r.pow(2);
    let pow_x = x.pow(2);
    let pow_y = y.pow(2);
    let exactly = ((pow_x + pow_y) % pow_r != 0) as i64;
    let pow_res = (pow_x + pow_y) / pow_r + ((pow_x + pow_y) % pow_r != 0) as i64;
    // for i in 0..1_000_000 {
    //     if i.pow(2) >= pow_res {
    //         res = i;
    //         break;
    //     }
    // }
    let mut res = ((pow_res as f64).sqrt() - 1.).floor() as i64;
    while res.pow(2) < pow_res {
        res += 1;
    }
    if (pow_x + pow_y) < pow_r {
        res += exactly;
    }

    println!("{}", res);
}
