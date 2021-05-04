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
        h: usize, w: usize,
        mut a: [[usize; w]; h]
    }
    // let mut last_odd_place_for_each_row = vec![w + 1; h];
    // let mut odd_num_row = vec![0; h];
    // let mut odd_num_col = vec![0; w];
    // for i in 0..h {
    //     for j in 0..w {
    //         if a[i][j] % 2 != 0 {
    //             odd_num_row[i] += 1;
    //             odd_num_col[j] += 1;
    //             last_odd_place_for_each_row[i] = j;
    //         }
    //     }
    // }
    let mut ops = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] % 2 != 0 {
                if j == w - 1 {
                    if i != h - 1 {
                        ops.push((i, j, i + 1, j));
                        a[i][j] -= 1;
                        a[i + 1][j] += 1;
                    }
                } else {
                    ops.push((i, j, i, j + 1));
                    a[i][j] -= 1;
                    a[i][j + 1] += 1;
                }
            }
        }
    }

    println!("{}", ops.len());
    for (x, y, xx, yy) in ops {
        println!("{} {} {} {}", x + 1, y + 1, xx + 1, yy + 1);
    }
}
