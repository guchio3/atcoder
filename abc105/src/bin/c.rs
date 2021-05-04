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
        mut n: i64
    }
    if n == 0 {
        println!("0");
        return;
    }

    let mut nums = vec![];
    let mut cum_nums = vec![];
    let mut num = 1;
    let mut pos_cum_num = 0;
    let mut neg_cum_num = 0;
    for _i in 0..32 {
        nums.push(num);
        if num > 0 {
            pos_cum_num += num;
            cum_nums.push(pos_cum_num)
        } else {
            neg_cum_num += num;
            cum_nums.push(neg_cum_num)
        }
        num *= -2;
    }

    let mut reses = vec![];
    let mut i = 0;
    while (n > 0 && cum_nums[i] < n) || (n < 0 && cum_nums[i] > n) {
        i += 1;
    }

    while i >= 2 {
        if n > 0 {
            if cum_nums[i] >= n && cum_nums[i - 2] < n {
                n -= nums[i];
                reses.push(1);
            } else {
                reses.push(0);
            }
        } else {
            if cum_nums[i] <= n && cum_nums[i - 2] > n {
                n -= nums[i];
                reses.push(1);
            } else {
                reses.push(0);
            }
        }
        i -= 1;
    }
    if n == 1 {
        reses.push(0);
        reses.push(1);
    } else if n == -2 {
        reses.push(1);
        reses.push(0);
    } else if n == -1 {
        reses.push(1);
        reses.push(1);
    } else {
        reses.push(0);
        reses.push(0);
    }

    println!("{}", reses.into_iter().join(""));

    // let mut num = 1;
    // while (n > 0 && num <= n) || (n < 0 && num >= n) {
    //     num *= -2;
    // }

    // let mut cnts = 0;
    // let mut reses = vec![];
    // while n != 0 && cnts < 10 {
    //     if (n > 0 && num >= n) || (n < 0 && num <= n) {
    //         n -= num;
    //         reses.push(1);
    //     } else {
    //         reses.push(0);
    //     }
    //     num /= -2;
    //     cnts += 1;
    //     println!("n: {} / num: {} / reses: {:?}", n, num, reses);
    // }

    // println!("{}", reses.into_iter().join(""));

    // let mut base_nums = vec![];
    // let mut num = 1;
    // for _i in 0..30 {
    //     base_nums.push(num);
    //     num *= -2;
    // }

    // for i in 0..(1 << 30) {
    //     let mut reses = vec![];
    //     let mut tmp_n: i64 = 0;
    //     for j in 0..30 {
    //         if i & (1 << j) > 0 {
    //             tmp_n += base_nums[j];
    //             reses.push(1u8);
    //         } else {
    //             reses.push(0u8);
    //         }
    //     }
    //     if tmp_n == n {
    //         reses = reses.into_iter().rev().collect();
    //         let mut j = 0;
    //         while reses[j] == 0 {
    //             j += 1;
    //         }
    //         println!("{}", reses[j..].into_iter().join(""));
    //         return;
    //     }
    // }
}
