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
        mut s: Chars,
        k: usize
    }
    let mut inner_num = 0;
    let mut bef_s_i = s[0];
    let mut i = 1;
    let mut stream = 1;
    let mut first_stream = 0;
    let mut last_stream = 0;
    while i < s.len() {
        let s_i = s[i];
        if s_i == bef_s_i {
            stream += 1;
        } else {
            if first_stream == 0 {
                first_stream = stream;
            }
            last_stream = stream;
            inner_num += stream / 2;
            stream = 1;
        }
        bef_s_i = s[min(i, s.len() - 1)];
        i += 1;
    }
    inner_num += stream / 2;
    if first_stream == 0 {
        first_stream = stream;
    }
    if stream > 1 {
        last_stream = stream;
    }
    if s[0] == s[s.len() - 1] {
        if (first_stream % 2 != 0) && (last_stream % 2 != 0) {
            inner_num += 1;
        }
    }
    if first_stream == s.len() {
        println!("{}", (stream * k) / 2);
    } else {
        let mut res = inner_num * k;
        if s[0] == s[s.len() - 1] && first_stream % 2 != 0 && last_stream % 2 != 0 {
            res -= 1;
        }
        println!("{}", res);
    }
}
