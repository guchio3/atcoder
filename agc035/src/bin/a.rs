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
        a: [usize; n]
    }
    let mut cnts = HashMap::new();
    for &a_i in a.iter() {
        *cnts.entry(a_i).or_insert(0) += 1;
    }
    if cnts.len() == 1 {
        if a[0] == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if n % 3 == 0 {
        if cnts.len() == 2 {
            if let Some(zero_num) = cnts.get(&0) {
                if *zero_num == n / 3 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            } else {
                println!("No");
            }
        } else if cnts.len() == 3 {
            let mut res = 0;
            for (key, value) in cnts {
                res ^= key;
                if !(value == n / 3) {
                    println!("No");
                    return;
                }
            }
            if res == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//     let mut all_xor = 0;
//     let mut cnts = HashMap::new();
//     for a_i in a {
//         all_xor ^= a_i;
//         *cnts.entry(a_i).or_insert(0) += 1;
//     }
//     let mut max_cnt = 0;
//     for (_key, value) in cnts {
//         max_cnt = max(max_cnt, value);
//     }
//     if all_xor == 0 {
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//     let mut aa = vec![a[n - 1]];
//     for i in 0..n {
//         aa.push(a[i]);
//     }
//     aa.push(a[0]);
//     for i in 1..n {
//         if aa[i] != (aa[i - 1] ^ aa[i + 1]) {
//             println!("{}", i);
//             println!("No");
//             return;
//         }
//     }
//     println!("Yes");
// }
