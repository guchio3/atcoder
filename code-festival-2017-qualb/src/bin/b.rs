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
        d: [usize; n],
        m: usize,
        t: [usize; m]
    }
    let mut cnt_dict = HashMap::new();
    for d_i in d {
        *(cnt_dict.entry(d_i).or_insert(0)) += 1;
    }
    for t_i in t {
        if let Some(cnt) = cnt_dict.get_mut(&t_i) {
            if *cnt > 0 {
                *cnt -= 1;
            } else {
                println!("NO");
                return;
            }
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
