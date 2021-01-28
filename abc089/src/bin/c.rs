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
        s: [String; n]
    }
    let march = vec!['M', 'A', 'R', 'C', 'H'];

    let mut march_cnt = HashMap::new();
    for &march_i in march.iter() {
        march_cnt.insert(march_i, 0);
    }
    for s_i in s {
        if let Some(value) = march_cnt.get_mut(&s_i.chars().nth(0).unwrap()) {
            *value += 1;
        }
    }

    let mut res: i64 = 0;
    for char_set in (0..5).combinations(3) {
        let mut sub_res = 1;
        for char_set_i in char_set {
            let char_i = march[char_set_i];
            sub_res *= march_cnt.get(&char_i).unwrap();
        }
        res += sub_res;
    }

    println!("{}", res);
}
