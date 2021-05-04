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
        s: Chars,
        t: Chars
    }
    let mut indices = vec![vec![]; 26];
    for i in 0..s.len() {
        let s_i = s[i];
        indices[s_i as usize - 'a' as usize].push(i + 1);
    }

    let mut res = 0;
    let mut cur_index = 0;
    let mut index_states = vec![0; 26];
    for t_i in t {
        let t_i_num = t_i as usize - 'a' as usize;
        if indices[t_i_num].len() == 0 {
            println!("-1");
            return;
        } else {
            while index_states[t_i_num] < indices[t_i_num].len()
                && cur_index > indices[t_i_num][index_states[t_i_num]]
            {
                index_states[t_i_num] += 1;
            }
            if index_states[t_i_num] >= indices[t_i_num].len() {
                res += s.len() - cur_index;
                cur_index = 0;
                index_states = vec![0; 26];
            }
            res += indices[t_i_num][index_states[t_i_num]] - cur_index;
            cur_index = indices[t_i_num][index_states[t_i_num]];
            index_states[t_i_num] += 1;
        }
    }

    println!("{}", res);
}

// fn main() {
//     input! {
//         s: Chars,
//         t: Chars
//     }
//     let mut indices = vec![vec![]; 26];
//     for i in 0..s.len() {
//         let s_i = s[i];
//         indices[s_i as usize - 'a' as usize].push(i);
//     }
//
//     let mut res = 1;
//     let mut cur_index = 0;
//     let mut index_states = vec![0; 26];
//     for t_i in t {
//         let t_i_num = t_i as usize - 'a' as usize;
//         if indices[t_i_num].len() == 0 {
//             println!("-1");
//             return;
//         } else {
//             while index_states[t_i_num] < indices[t_i_num].len()
//                 && cur_index < indices[t_i_num][index_states[t_i_num]]
//             {
//                 index_states[t_i_num] += 1;
//             }
//             if index_states[t_i_num] >= indices[t_i_num].len() {
//                 res += 1;
//                 index_states = vec![0; 26];
//                 cur_index = indices[t_i_num][0];
//                 index_states[t_i_num] += 1;
//             } else {
//                 cur_index = indices[t_i_num][index_states[t_i_num]];
//                 index_states[t_i_num] += 1;
//             }
//         }
//     }
//
//     println!("{}", res + 1);
// }
