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
        mut a: [usize; n]
    }
    a.sort();
    let mut i = n / 2;
    let mut j_left = 0;
    let mut j_right = n - 1;
    let mut j_cnt = 1;
    let mut j_dir = true;
    let mut diff = -1;
    let mut res_vec = vec![0; n];
    for _ in 0..n {
        if j_dir {
            res_vec[i] = a[j_left];
            j_left += 1;
        } else {
            res_vec[i] = a[j_right];
            j_right -= 1;
        }
        i = (i as i64 + diff) as usize;
        if diff < 0 {
            diff -= 1;
        } else {
            diff += 1;
        }
        diff *= -1;
        if j_cnt != 0 {
            j_dir = !j_dir;
        }
        j_cnt = (j_cnt + 1) % 2;
    }
    // println!("{:?}", res_vec);
    let mut res1 = 0;
    let mut bef_res_vec_i = res_vec[0];
    for res_vec_i in res_vec {
        res1 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
        bef_res_vec_i = res_vec_i;
    }

    a.sort_by(|x, y| y.partial_cmp(&x).unwrap());
    let mut i = n / 2;
    let mut j_left = 0;
    let mut j_right = n - 1;
    let mut j_cnt = 1;
    let mut j_dir = true;
    let mut diff = -1;
    let mut res_vec = vec![0; n];
    for _ in 0..n {
        if j_dir {
            res_vec[i] = a[j_left];
            j_left += 1;
        } else {
            res_vec[i] = a[j_right];
            j_right -= 1;
        }
        i = (i as i64 + diff) as usize;
        if diff < 0 {
            diff -= 1;
        } else {
            diff += 1;
        }
        diff *= -1;
        if j_cnt != 0 {
            j_dir = !j_dir;
        }
        j_cnt = (j_cnt + 1) % 2;
    }
    // println!("{:?}", res_vec);
    let mut res2 = 0;
    let mut bef_res_vec_i = res_vec[0];
    for res_vec_i in res_vec {
        res2 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
        bef_res_vec_i = res_vec_i;
    }

    println!("{}", max(res1, res2));
}

// fn main() {
//     input! {
//         n: usize,
//         mut a: [usize; n]
//     }
//     a.sort();
//     let mut i = n / 2;
//     let mut j_left = 0;
//     let mut j_right = n - 1;
//     let mut j_cnt = 1;
//     let mut j_dir = true;
//     let mut diff = -1;
//     let mut res_vec = vec![0; n];
//     for _ in 0..n {
//         if j_dir {
//             res_vec[i] = a[j_left];
//             j_left += 1;
//         } else {
//             res_vec[i] = a[j_right];
//             j_right -= 1;
//         }
//         i = (i as i64 + diff) as usize;
//         if diff < 0 {
//             diff -= 1;
//         } else {
//             diff += 1;
//         }
//         diff *= -1;
//         if j_cnt != 0 {
//             j_dir = !j_dir;
//         }
//         j_cnt = (j_cnt + 1) % 2;
//     }
//     // println!("{:?}", res_vec);
//     let mut res1 = 0;
//     let mut bef_res_vec_i = res_vec[0];
//     for res_vec_i in res_vec {
//         res1 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
//         bef_res_vec_i = res_vec_i;
//     }
//
//     a.sort_by(|x, y| y.partial_cmp(&x).unwrap());
//     let mut i = n / 2;
//     let mut j_left = 0;
//     let mut j_right = n - 1;
//     let mut j_cnt = 1;
//     let mut j_dir = true;
//     let mut diff = -1;
//     let mut res_vec = vec![0; n];
//     for _ in 0..n {
//         if j_dir {
//             res_vec[i] = a[j_left];
//             j_left += 1;
//         } else {
//             res_vec[i] = a[j_right];
//             j_right -= 1;
//         }
//         i = (i as i64 + diff) as usize;
//         if diff < 0 {
//             diff -= 1;
//         } else {
//             diff += 1;
//         }
//         diff *= -1;
//         if j_cnt != 0 {
//             j_dir = !j_dir;
//         }
//         j_cnt = (j_cnt + 1) % 2;
//     }
//     // println!("{:?}", res_vec);
//     let mut res2 = 0;
//     let mut bef_res_vec_i = res_vec[0];
//     for res_vec_i in res_vec {
//         res2 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
//         bef_res_vec_i = res_vec_i;
//     }
//
//     println!("{}", max(res1, res2));
// }

// fn main() {
//     input! {
//         n: usize,
//         mut a: [usize; n]
//     }
//     a.sort();
//
//     let mut res_vec = vec![];
//     for i in 0..n / 2 + (n % 2 != 0) as usize {
//         if n % 2 != 0 && i == n / 2 {
//             res_vec.push(a[i]);
//         } else {
//             res_vec.push(a[i]);
//             res_vec.push(a[n - i - 1]);
//         }
//     }
//
//     let mut res1 = 0;
//     let mut bef_res_vec_i = res_vec[0];
//     for res_vec_i in res_vec {
//         res1 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
//         bef_res_vec_i = res_vec_i;
//     }
//
//     a.sort_by(|x, y| y.partial_cmp(&x).unwrap());
//
//     let mut res_vec = vec![];
//     for i in 0..n / 2 + (n % 2 != 0) as usize {
//         if n % 2 != 0 && i == n / 2 {
//             res_vec.push(a[i]);
//         } else {
//             res_vec.push(a[i]);
//             res_vec.push(a[n - i - 1]);
//         }
//     }
//
//     let mut res2 = 0;
//     let mut bef_res_vec_i = res_vec[0];
//     for res_vec_i in res_vec {
//         res2 += max(bef_res_vec_i, res_vec_i) - min(bef_res_vec_i, res_vec_i);
//         bef_res_vec_i = res_vec_i;
//     }
//
//     println!("{}", max(res1, res2));
// }
