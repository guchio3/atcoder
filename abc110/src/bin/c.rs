#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut dict1 = HashMap::new();
    let mut dict2 = HashMap::new();
    let mut cnt_dict1 = HashMap::new();
    let mut cnt_dict2 = HashMap::new();
    for s_i in s {
        *(dict1.entry(s_i).or_insert(0)) += 1;
    }
    for t_i in t {
        *(dict2.entry(t_i).or_insert(0)) += 1;
    }

    for (_key1, value1) in dict1 {
        *(cnt_dict1.entry(value1).or_insert(0)) += 1;
    }
    for (_key2, value2) in dict2 {
        *(cnt_dict2.entry(value2).or_insert(0)) += 1;
    }

    for cnt in cnt_dict1.keys() {
        if !cnt_dict2.contains_key(&cnt) {
            println!("No");
            return;
        } else if cnt_dict1.get(&cnt).unwrap() != cnt_dict2.get(&cnt).unwrap() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
