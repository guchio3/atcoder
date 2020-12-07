#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars
    }

    let keyence: Vec<char> = String::from("keyence").chars().into_iter().collect();
    if s[..7].to_vec() == keyence || s[s.len() - 7..].to_vec() == keyence {
        println!("YES")
    } else {
        let mut i = 0;
        while s[i] == keyence[i] {
            i += 1;
        }
        while i < keyence.len() && s[s.len() - keyence.len() + i] == keyence[i] {
            i += 1;
        }
        if i == keyence.len() {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    // 'first: for i in 0..s.len() {
    //     let mut used = false;
    //     let mut switched = i != 0;
    //     // if i + keyence.len() > s.len() {
    //     //     break;
    //     // }

    //     let mut j = 0;
    //     let mut k = 0;
    //     while j < keyence.len() {
    //         if i + k >= s.len() {
    //             continue 'first;
    //         }

    //         if s[i + k] != keyence[j] {
    //             if switched {
    //                 continue 'first;
    //             } else {
    //                 used = true;
    //             }
    //         } else {
    //             if used {
    //                 switched = true;
    //             }
    //             j += 1;
    //         }
    //         k += 1;
    //     }
    //     println!("YES");
    //     return;
    // }

    // println!("NO");
}
