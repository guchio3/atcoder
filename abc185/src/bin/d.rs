#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [i64; m]
    }
    a.sort();
    let mut wid_cnts = HashMap::new();
    let mut bef_a_i = 0;
    for a_i in a {
        let tmp = a_i - bef_a_i - 1;
        if tmp != 0 {
            *wid_cnts.entry(tmp).or_insert(0) += 1;
        }
        bef_a_i = a_i;
    }
    let tmp = (n + 1) as i64 - bef_a_i - 1;
    if tmp != 0 {
        *wid_cnts.entry(tmp).or_insert(0) += 1;
    }

    if wid_cnts.len() == 0 {
        println!("0");
        return;
    }

    let mut min_key = n as i64;
    for &key in wid_cnts.keys() {
        min_key = min(key, min_key);
    }

    let mut res = 0;
    for &key in wid_cnts.keys() {
        let value = wid_cnts.get(&key).unwrap();
        let unit = key / min_key + (key % min_key != 0) as i64;
        res += unit * value;
    }

    println!("{}", res);

    // println!("{:?}", wid_cnts);

    // let mut bef_key = -1;
    // let mut tmp_gcd = -1;
    // for &key in wid_cnts.keys() {
    //     if bef_key == -1 {
    //         tmp_gcd = key;
    //     } else {
    //         tmp_gcd = gcd(key, bef_key);
    //     }
    //     bef_key = key;
    // }

    // // println!("{}", tmp_gcd);

    // let mut res = 0;
    // for key in wid_cnts.keys() {
    //     res += key / tmp_gcd;
    // }
    // println!("{}", res)
}
