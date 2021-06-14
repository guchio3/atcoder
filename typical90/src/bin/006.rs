#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::iter::FromIterator;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize, k: usize,
        s: Chars
    }
    let mut next_indices = vec![vec![INF; n]; 26];
    next_indices[s[n - 1] as usize - 'a' as usize][n - 1] = n - 1;
    for i in (0..n - 1).rev() {
        for j in 0..26 {
            next_indices[j][i] = next_indices[j][i + 1];
        }
        next_indices[s[i] as usize - 'a' as usize][i] = i;
    }

    // for i in 0..26 {
    //     println!(
    //         "{}: {:?}\n",
    //         (i + 'a' as usize) as u8 as char,
    //         next_indices[i]
    //     );
    // }

    let mut sub_s = vec![];
    let mut i = 0;
    'outer: while sub_s.len() < k {
        for j in 0..26 {
            if next_indices[j][i] != INF && n - 1 - next_indices[j][i] >= k - sub_s.len() - 1 {
                sub_s.push(s[next_indices[j][i]]);
                i = next_indices[j][i] + 1;
                continue 'outer;
            }
        }
    }
    println!("{}", String::from_iter(sub_s));
}

// fn main() {
//     input! {
//         n: usize, k: usize,
//         s: Chars
//     }
//     let mut sub_s = VecDeque::new();
//     for i in (n - k)..n {
//         sub_s.push_back(s[i]);
//     }
//
//     for i in (0..(n - k)).rev() {
//         if s[i] <= sub_s[0] {
//             sub_s.push_front(s[i]);
//             for j in 0..k {
//                 if sub_s[j + 1] < sub_s[j] {
//                     sub_s.remove(j);
//                     break;
//                 }
//             }
//             // 一つも remove できなかった場合はお尻を除去
//             if sub_s.len() > k {
//                 sub_s.pop_back();
//             }
//         }
//     }
//
//     println!("{}", String::from_iter(sub_s));
// }
//
// const INF: usize = 1_000_000_000;
//
// fn main() {
//     input! {
//         n: usize, k: usize,
//         s: Chars
//     }
//     let mut sub_s = LinkedList::new();
//     for i in ((n - k)..n).rev() {
//         sub_s.push_front(s[i]);
//     }
//
//     let mut next_change_index = 0;
//     let mut next_change_iter = sub_s.iter_mut();
//
//     for i in (0..(n - k)).rev() {
//         if s[i] < *sub_s.front().unwrap() {
//             sub_s.push_front(s[i]);
//             next_change_index += 1;
//             if next_change_index < sub_s.len() {
//                 sub_s.remove()
//             }
//             next_change_index -= 1;
//             for j in 0..k {
//                 if sub_s[j + 1] < sub_s[j] {
//                     sub_s.remove(j);
//                     break;
//                 }
//             }
//             // 一つも remove できなかった場合はお尻を除去
//             if sub_s.len() > k {
//                 sub_s.pop_back();
//             }
//             worse_than_head_index = calc_worse_than_head_index(&sub_s, 1);
//         } else if s[i] == sub_s[0] {
//             if worse_than_head_index != INF {
//                 sub_s[worse_than_head_index] = s[i];
//                 worse_than_head_index = calc_worse_than_head_index(&sub_s, worse_than_head_index);
//             }
//         }
//     }
//
//     println!("{}", String::from_iter(sub_s));
// }
//
// const INF: usize = 1_000_000_000;
//
// fn calc_worse_than_head_index(sub_s: &VecDeque<char>, start: usize) -> usize {
//     let mut res = INF;
//     let mut i = start;
//     while i < sub_s.len() && sub_s[i - 1] == sub_s[0] {
//         if sub_s[i] > sub_s[0] {
//             res = i;
//             break;
//         }
//         i += 1;
//     }
//     res
// }
//
// fn main() {
//     input! {
//         n: usize, k: usize,
//         s: Chars
//     }
//     let mut sub_s = VecDeque::new();
//     for i in (n - k)..n {
//         sub_s.push_back(s[i]);
//     }
//
//     let mut worse_than_head_index = calc_worse_than_head_index(&sub_s, 1);
//
//     for i in (0..(n - k)).rev() {
//         if s[i] < sub_s[0] {
//             sub_s.push_front(s[i]);
//             for j in 0..k {
//                 if sub_s[j + 1] < sub_s[j] {
//                     sub_s.remove(j);
//                     break;
//                 }
//             }
//             // 一つも remove できなかった場合はお尻を除去
//             if sub_s.len() > k {
//                 sub_s.pop_back();
//             }
//             worse_than_head_index = calc_worse_than_head_index(&sub_s, 1);
//         } else if s[i] == sub_s[0] {
//             if worse_than_head_index != INF {
//                 sub_s[worse_than_head_index] = s[i];
//                 worse_than_head_index = calc_worse_than_head_index(&sub_s, worse_than_head_index);
//             }
//         }
//     }
//
//     println!("{}", String::from_iter(sub_s));
// }
