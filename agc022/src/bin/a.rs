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
        mut s: Chars
    }
    let mut used_alphabets = vec![false; 26];
    for &s_i in s.iter() {
        used_alphabets[s_i as usize - 'a' as usize] = true;
    }
    if s.len() == 26 {
        let mut i = 25;
        let mut bef_char_num = 0;
        loop {
            let char_num = s[i] as usize - 'a' as usize;
            if char_num < bef_char_num {
                let mut min_char_num = 'z' as usize + 1;
                for j in i + 1..26 {
                    let tmp_char_num = s[j] as usize - 'a' as usize;
                    if tmp_char_num > char_num {
                        min_char_num = min(min_char_num, tmp_char_num);
                    }
                }
                let mut res_vec = vec![];
                for j in 0..i {
                    res_vec.push(s[j]);
                }
                res_vec.push((min_char_num + 'a' as usize) as u8 as char);
                println!("{}", res_vec.into_iter().join(""));
                return;
            }
            if i > 0 {
                bef_char_num = char_num;
                i -= 1;
            } else {
                println!("-1");
                return;
            }
        }
    } else {
        let mut target_alphabet = 'a';
        for i in 0..26 {
            if !used_alphabets[i] {
                target_alphabet = ('a' as u8 + i as u8) as char;
                break;
            }
        }
        println!("{}{}", s.into_iter().join(""), target_alphabet);
    }
}
