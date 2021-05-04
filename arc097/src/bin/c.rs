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
        s: Chars,
        mut k: usize
    }
    for i in 0..26 {
        let mut used = HashSet::new();
        let target_char = (i as u8 + 'a' as u8) as char;
        for j in 0..s.len() {
            let index = s.len() - j - 1;
            if s[index] == target_char {
                let mut tmp_k = k;
                let mut tmp_string = String::from("");
                for l in 0..=j {
                    if tmp_k == 0 {
                        break;
                    } else {
                        tmp_string.push(s[index + l]);
                        used.insert(tmp_string.clone());
                        tmp_k -= 1;
                    }
                }
            }
        }
        let mut used_vec: Vec<String> = used.into_iter().collect();
        if used_vec.len() >= k {
            used_vec.sort();
            for res_string in used_vec {
                k -= 1;
                if k == 0 {
                    println!("{}", res_string);
                    return;
                }
            }
        } else {
            k -= used_vec.len();
        }
    }
}
