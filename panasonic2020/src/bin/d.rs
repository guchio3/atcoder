#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn dfs(n: usize, tmp_res: &mut Vec<usize>, max_num: usize, reses: &mut Vec<Vec<usize>>) {
    if tmp_res.len() < n {
        for i in 0..=max_num + 1 {
            tmp_res.push(i);
            if i > max_num {
                dfs(n, tmp_res, max_num + 1, reses);
            } else {
                dfs(n, tmp_res, max_num, reses);
            }
            tmp_res.pop();
        }
    } else {
        reses.push(tmp_res.clone());
    }
}

fn main() {
    input! {
        n: usize
    }
    let mut reses = vec![];
    dfs(n, &mut vec![0], 0, &mut reses);
    for res in reses {
        println!(
            "{}",
            res.into_iter()
                .map(|x| (x as u8 + 'a' as u8) as char)
                .join("")
        );
    }
}

// fn dfs(cumnum: usize, n: usize, tmp_res: &mut Vec<usize>, reses: &mut HashSet<Vec<usize>>) {
//     if cumnum < n {
//         'outer: for i in 0..n {
//             for j in 0..i {
//                 if tmp_res[i] + 1 > tmp_res[j] {
//                     continue 'outer;
//                 }
//             }
//             tmp_res[i] += 1;
//             dfs(cumnum + 1, n, tmp_res, reses);
//             tmp_res[i] -= 1;
//         }
//     } else {
//         reses.insert(tmp_res.to_vec());
//     }
// }
//
// fn main() {
//     input! {
//         n: usize
//     }
//     let mut tmp_reses: HashSet<Vec<usize>> = HashSet::new();
//     let mut reses: HashSet<Vec<usize>> = HashSet::new();
//     dfs(0, n, &mut vec![0; n], &mut tmp_reses);
//     let mut perm_base = vec![];
//     for res in tmp_reses {
//         let mut tmp_perm_base = vec![];
//         for i in 0..n {
//             for _j in 0..res[i] {
//                 tmp_perm_base.push(i);
//             }
//         }
//         perm_base.push(tmp_perm_base);
//     }
//     for perm_base_i in perm_base {
//         for mut permed in perm_base_i.into_iter().permutations(n) {
//             let mut mapped = vec![n; n];
//             let mut cnt = 0;
//             for i in 0..n {
//                 if mapped[permed[i]] != n {
//                     permed[i] = mapped[permed[i]];
//                 } else {
//                     permed[i] = cnt;
//                     mapped[i] = cnt;
//                     cnt += 1;
//                 }
//             }
//             reses.insert(permed);
//         }
//     }
//     let mut reses: Vec<Vec<usize>> = reses.into_iter().collect();
//     reses.sort();
//     for res in reses {
//         println!(
//             "{}",
//             res.into_iter()
//                 .map(|x| (x as u8 + 'a' as u8) as char)
//                 .join("")
//         );
//     }
// }

// fn dfs(
//     alphabet_num: usize,
//     cumnum: usize,
//     n: usize,
//     tmp_res: &mut Vec<usize>,
//     reses: &mut HashSet<Vec<usize>>,
// ) {
//     if cumnum < n {
//         for i in 0..alphabet_num {
//             tmp_res[i] += 1;
//             dfs(alphabet_num, cumnum + 1, n, tmp_res, reses);
//             tmp_res[i] -= 1;
//         }
//     } else {
//         reses.insert(tmp_res.to_vec());
//     }
// }
//
// fn main() {
//     input! {
//         n: usize
//     }
//     let mut reses: HashSet<Vec<usize>> = HashSet::new();
//     for alphabet_num in 1..=n {
//         let mut tmp_reses = HashSet::new();
//         dfs(
//             alphabet_num,
//             alphabet_num,
//             n,
//             &mut vec![1; alphabet_num],
//             &mut tmp_reses,
//         );
//         let mut perm_base = vec![];
//         for tmp_res in tmp_reses {
//             let mut tmp_perm_base = vec![];
//             for i in 0..alphabet_num {
//                 for _j in 0..tmp_res[i] {
//                     tmp_perm_base.push(i);
//                 }
//             }
//             perm_base.push(tmp_perm_base);
//         }
//         for perm_base_i in perm_base {
//             for mut permed in perm_base_i.into_iter().permutations(n) {
//                 permed.sort();
//                 reses.insert(permed);
//             }
//         }
//     }
//     let mut reses: Vec<Vec<usize>> = reses.into_iter().collect();
//     reses.sort();
//     for res in reses {
//         println!(
//             "{}",
//             res.into_iter()
//                 .map(|x| (x as u8 + 'a' as u8) as char)
//                 .join("")
//         );
//     }
// }
