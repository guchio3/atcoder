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
        s1: Chars,
        s2: Chars,
        s3: Chars
    }
    if s3.len() < s1.len() || s3.len() < s2.len() {
        println!("UNSOLVABLE");
        return;
    }
    let mut exist_chars = HashSet::new();
    for &s1_i in s1.iter() {
        exist_chars.insert(s1_i);
    }
    for &s2_i in s2.iter() {
        exist_chars.insert(s2_i);
    }
    for &s3_i in s3.iter() {
        exist_chars.insert(s3_i);
    }
    if exist_chars.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let exist_chars: Vec<char> = exist_chars.into_iter().collect();
    let mut char_ids = HashMap::new();
    for i in 0..exist_chars.len() {
        char_ids.insert(exist_chars[i], i);
    }
    let perm_nums = (0..10).into_iter().permutations(exist_chars.len());
    for nums_i in perm_nums {
        let mut s3_num: usize = 0;
        let mut s3_chars = vec![];
        let mut digit: usize = 1;
        for i in (0..s3.len()).rev() {
            let char_id: usize = *char_ids.get(&s3[i]).unwrap();
            let num = nums_i[char_id];
            s3_chars.push(num);
            s3_num += num * digit;
            digit *= 10;
        }
        let mut s2_num: usize = 0;
        let mut digit = 1;
        let mut s2_chars = vec![];
        for i in (0..s2.len()).rev() {
            let char_id: usize = *char_ids.get(&s2[i]).unwrap();
            let num = nums_i[char_id];
            s2_chars.push(num);
            s2_num += num * digit;
            digit *= 10;
        }
        let mut s1_num = 0;
        let mut digit = 1;
        let mut s1_chars = vec![];
        for i in (0..s1.len()).rev() {
            let char_id: usize = *char_ids.get(&s1[i]).unwrap();
            let num = nums_i[char_id];
            s1_chars.push(num);
            s1_num += num * digit;
            digit *= 10;
        }
        if s1_chars[s1_chars.len() - 1] != 0
            && s2_chars[s2_chars.len() - 1] != 0
            && s3_chars[s3_chars.len() - 1] != 0
            && s3_num == s1_num + s2_num
        {
            println!("{}", s1_chars.into_iter().rev().join(""));
            println!("{}", s2_chars.into_iter().rev().join(""));
            println!("{}", s3_chars.into_iter().rev().join(""));
            return;
        }
    }
    println!("UNSOLVABLE");
}
