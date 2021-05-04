#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    let mut cnts = vec![vec![0; 13]; s.len()];
    if s[s.len() - 1] == '?' {
        for i in 0..=9 {
            cnts[n - 1][i] += 1;
        }
    } else {
        cnts[n - 1][s[n - 1] as usize - '0' as usize] += 1
    }

    let mut mod_digit = 10;
    for i in (0..s.len() - 1).rev() {
        let s_i = s[i];
        if s_i == '?' {
            let current_nums = 0..=9;
            for current_num in current_nums {
                for index in 0..13 {
                    let bef_cnts = cnts[i + 1][index];
                    cnts[i][(index + mod_digit * current_num) % 13] += bef_cnts;
                    cnts[i][(index + mod_digit * current_num) % 13] %= MOD;
                }
            }
        } else {
            let current_num = s[i] as usize - '0' as usize;
            for index in 0..13 {
                let bef_cnts = cnts[i + 1][index];
                cnts[i][(index + mod_digit * current_num) % 13] += bef_cnts;
                cnts[i][(index + mod_digit * current_num) % 13] %= MOD;
            }
        }
        mod_digit = (mod_digit * 10) % 13;
    }

    println!("{}", cnts[0][5]);
}
