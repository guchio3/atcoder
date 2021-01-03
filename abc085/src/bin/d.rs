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
        n: usize, h: usize,
        mut ab: [(usize, usize); n]
    }
    ab.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    let mut cum_vec_dict = vec![];
    let mut i = 0;
    for &(_a, b) in ab.iter() {
        if cum_vec_dict.len() == 0 {
            cum_vec_dict.push((b, b, 1));
        } else if cum_vec_dict[i].0 != b {
            cum_vec_dict.push((b, b, 1));
            i += 1;
        } else {
            cum_vec_dict[i].1 += b;
            cum_vec_dict[i].2 += 1;
        }
    }

    ab.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
    let mut res = h;
    let mut cum_i = 0;
    let mut cum_sum = 0;
    let mut cum_used_num = 0;
    'outer: for &(a, _b) in ab.iter() {
        while cum_i < cum_vec_dict.len() && cum_vec_dict[cum_i].0 >= a {
            if cum_sum + cum_vec_dict[cum_i].1 >= h {
                let mut tmp_res = cum_used_num;
                for j in 1..=cum_vec_dict[cum_i].2 {
                    tmp_res += 1;
                    if cum_sum + cum_vec_dict[cum_i].0 * j >= h {
                        break;
                    }
                }
                res = min(res, tmp_res);
                break 'outer;
            }
            cum_sum += cum_vec_dict[cum_i].1;
            cum_used_num += cum_vec_dict[cum_i].2;
            cum_i += 1;
        }
        // println!("res: {} / cum_sum: {} / cum_used_num: {}", res, cum_sum, cum_used_num);
        let h_minus_b_sum = h - cum_sum;
        res = min(
            res,
            min(cum_used_num, n) + h_minus_b_sum / a + (h_minus_b_sum % a != 0) as usize,
        );
    }
    println!("{}", res);
}
