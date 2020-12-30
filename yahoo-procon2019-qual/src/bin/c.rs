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
        mut k: usize, a: usize, b: usize
    }
    if b <= a + 2 || k < a {
        println!("{}", k + 1);
    } else {
        println!("{}", a + (k - a + 1) / 2 * (b - a) + ((k - a + 1) % 2) as usize);
    }
}


// fn main() {
//     input! {
//         mut k: usize, a: usize, b: usize
//     }
//     if b <= a + 2 || k < a {
//         println!("{}", k + 1);
//     } else {
//         let mut num = 1;
//         while k > 0 {
//             if num < a {
//                 let a_diff = a - num;
//                 num += min(a_diff, k);
//                 k -= min(a_diff, k);
//             }
//             if k == 0 {
//                 break;
//             } else if k == 1 {
//                 num += 1;
//                 break;
//             }
//             num += b - a;
//             k -= 2;
//         }
//         println!("{}", num);
//         // k -= a - 1;
//         // println!("{}", 1 + (b - a) * k / (a + 1) + (k % (a + 1)) as usize);
//     }
// }
