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
        n: usize,
        s1: Chars,
        s2: Chars
    }
    let mut domino_chars = vec![vec![' '; n]; 2];
    for i in 0..n {
        domino_chars[0][i] = s1[i];
        domino_chars[1][i] = s2[i];
    }

    let mut res = 1;
    let mut i = 0;
    while i < n {
        if i == 0 {
            if n == 1 {
                res = res * 3;
            } else {
                if domino_chars[0][i] == domino_chars[0][i + 1] {
                    res = (res * 6) % MOD;
                    i += 1;
                } else {
                    res = (res * 3) % MOD;
                }
            }
        } else if i < n - 1 {
            if domino_chars[0][i] == domino_chars[0][i + 1] {
                if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
                    res = (res * 2) % MOD;
                } else {
                    res = (res * 3) % MOD;
                }
                i += 1;
            } else {
                if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
                    res = (res * 2) % MOD;
                } else {
                    res = (res * 1) % MOD;
                }
            }
        } else {
            if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
                res = (res * 2) % MOD;
            } else {
                res = (res * 1) % MOD;
            }
        }
        i += 1;
    }
    println!("{}", res);

    //    let mut res = 1;
    //    let mut nums = vec![vec![1_000; n]; 2];
    //    for i in 0..n {
    //        if nums[0][i] == 1_000 {
    //            let mut res_num = 3;
    //            if domino_chars[0][i] == domino_chars[1][i] {
    //                if i > 0 {
    //                    if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
    //                        res_num -= 1;
    //                    } else {
    //                        res_num -= 2;
    //                    }
    //                }
    //                nums[0][i] = res_num;
    //                nums[1][i] = res_num;
    //            } else {
    //                if i > 0 {
    //                    if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
    //                        res_num -= 1;
    //                    } else {
    //                        res_num -= 1;
    //                    }
    //                }
    //                nums[0][i] = res_num;
    //                nums[0][i + 1] = res_num;
    //            }
    //            res = (res * res_num) % MOD;
    //        }
    //        if nums[1][i] == 1_000 {
    //            let mut res_num = 3;
    //            if i > 0 {
    //                if domino_chars[0][i - 1] == domino_chars[1][i - 1] {
    //                    res_num -= 2;
    //                } else {
    //                    res_num -= 1;
    //                }
    //            } else {
    //                res_num -= 1;
    //            }
    //            nums[1][i] = res_num;
    //            nums[1][i + 1] = res_num;
    //            res = (res * res_num) % MOD;
    //        }
    //    }
    //    println!("{:?}", nums);
    //    println!("{}", res);
} //
