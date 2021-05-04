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
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }
    a.sort();
    b.sort();
    c.sort();
    let mut b_possible_nums = vec![0; n];
    let mut c_possible_nums = vec![0; n];
    for i in 0..n {
        let b_i = b[i];
        let b_i_binary_search_res = a.binary_search(&b_i);
        let b_i_binary_search_res_minus = a.binary_search(&(b_i - 1));
        match b_i_binary_search_res {
            Ok(_) => match b_i_binary_search_res_minus {
                Ok(numnum) => {
                    b_possible_nums[i] = numnum + 1;
                }
                Err(numnum) => {
                    b_possible_nums[i] = numnum;
                }
            },
            Err(num) => {
                b_possible_nums[i] = num;
            }
        }

        let c_i = c[i];
        let c_i_binary_search_res = b.binary_search(&c_i);
        let c_i_binary_search_res_minus = b.binary_search(&(c_i - 1));
        match c_i_binary_search_res {
            Ok(_) => match c_i_binary_search_res_minus {
                Ok(numnum) => {
                    c_possible_nums[i] = numnum + 1;
                }
                Err(numnum) => {
                    c_possible_nums[i] = numnum;
                }
            },
            Err(num) => {
                c_possible_nums[i] = num;
            }
        }
    }

    let mut res: usize = 0;
    let mut i = 0;
    let mut j = 0;
    let mut b_cumsum = 0;
    while i < n {
        while j < c_possible_nums[i] {
            b_cumsum += b_possible_nums[j];
            j += 1;
        }
        res += b_cumsum;
        i += 1;
    }

    println!("{}", res);
}
