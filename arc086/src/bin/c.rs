#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut cnt_dict = HashMap::new();
    for a_i in a {
        *cnt_dict.entry(a_i).or_insert(0) += 1;
    }

    // let mut largest_value = 0;
    // let mut largest_key = 0;
    let mut key_value_vec = vec![];
    for (key, value) in cnt_dict {
        key_value_vec.push((key, value));
        // if value > largest_value {
        //     largest_key = key;
        //     largest_value = value;
        // }
    }
    key_value_vec.sort_by_key(|&pair| pair.1);

    let mut res = 0;
    let mut used_unique = 0;
    for &key_value in key_value_vec.iter() {
        if key_value_vec.len() - used_unique <= k {
            break;
        }
        res += key_value.1;
        used_unique += 1;
    }

    println!("{}", res);
}
