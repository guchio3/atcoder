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
    let mut res: usize = 0;
    for i in 0..n - 1 {
        let odd_even_pair = (a[i] % 2, a[i + 1] % 2);
        let pair_minus_num;
        match odd_even_pair {
            (0, 0) => pair_minus_num = min(a[i], a[i + 1]),
            (0, 1) => pair_minus_num = min(a[i], a[i + 1]) / 2 * 2,
            (1, 0) => {
                if a[i] > a[i + 1] {
                    if a[i + 1] == 0 {
                        pair_minus_num = 0;
                    } else {
                        pair_minus_num = a[i + 1] - 1;
                    }
                } else {
                    pair_minus_num = a[i];
                }
            }
            (1, 1) => pair_minus_num = min(a[i], a[i + 1]),
            (_, _) => panic!(),
        }

        res += pair_minus_num;
        a[i] -= pair_minus_num;
        a[i + 1] -= pair_minus_num;
        res += a[i] / 2;
    }
    res += a[n - 1] / 2;
    println!("{}", res);
}
