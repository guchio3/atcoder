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
        n: usize,
        d: [usize; n]
    }

    let mut res = 0;
    for i in 0..n {
        let mut i_clone = i + 1;
        while i_clone % 10 == ((i + 1) % 10) {
            i_clone /= 10;
        }
        if i_clone != 0 {
            continue;
        }
        for j in 1..=d[i] {
            let mut j_clone = j;
            while j_clone % 10 == ((i + 1) % 10) {
                j_clone /= 10;
            }
            if j_clone == 0 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
