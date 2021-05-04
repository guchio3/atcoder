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
        abcd: Chars
    }
    for i in 0..1 << 3 {
        let mut tmp_res = abcd[0] as i64 - '0' as i64;
        let mut res_vec = vec![abcd[0]; 1];
        for j in 0..3 {
            if i & 1 << j > 0 {
                tmp_res += abcd[j + 1] as i64 - '0' as i64;
                res_vec.push('+');
            } else {
                tmp_res -= abcd[j + 1] as i64 - '0' as i64;
                res_vec.push('-');
            }
            res_vec.push(abcd[j + 1]);
        }
        if tmp_res == 7 {
            res_vec.push('=');
            res_vec.push('7');
            println!("{}", res_vec.into_iter().join(""));
            return;
        }
    }
}
