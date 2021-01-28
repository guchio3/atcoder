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
    }
    // 不親切でない → 正直者の可能性がある
    // → 不親切でないものを探す
    // → 矛盾しない人何人いるか
    let mut matrix = vec![vec![-1; n]; n];
    for i in 0..n {
        input! {
            a_i: usize,
            xy_i: [(usize, i64); a_i]
        }
        for xy_i_j in xy_i {
            matrix[i][xy_i_j.0 - 1] = xy_i_j.1;
        }
    }

    let mut res = 0;
    let all_men: HashSet<usize> = (0..n).into_iter().collect();
    for i in 0..(1 << n) {
        let mut tmp_res = 0;
        // let mut dict_vec = HashMap::new();
        let mut true_men = HashSet::new();
        let mut said_true_men = HashSet::new();
        let mut said_false_men = HashSet::new();
        for j in 0..n {
            if i & (1 << j) > 0 {
                true_men.insert(j);
                tmp_res += 1;
                for k in 0..n {
                    if matrix[j][k] == 1 {
                        said_true_men.insert(k);
                    } else if matrix[j][k] == 0 {
                        said_false_men.insert(k);
                    }
                    // if matrix[j][k] != -1 {
                    //     (*dict_vec.entry(k).or_insert(vec![])).push(matrix[j][k]);
                    // }
                }
            }
        }
        let false_men: HashSet<usize> = all_men
            .difference(&true_men)
            .into_iter()
            .map(|x| *x)
            .collect();
        let muzyun_vec: Vec<usize> = true_men
            .intersection(&said_false_men)
            .into_iter()
            .map(|x| *x)
            .collect();
        let muzyun_vec2: Vec<usize> = false_men
            .intersection(&said_true_men)
            .into_iter()
            .map(|x| *x)
            .collect();
        if muzyun_vec.len() == 0 && muzyun_vec2.len() == 0 {
            if tmp_res > res {
                // println!("-----------------------------------------");
                // println!("{}", i);
                // println!("all_men: {:?}", all_men);
                // println!("true_men: {:?}", true_men);
                // println!("false_men: {:?}", false_men);
                // println!("said_true_men: {:?}", said_true_men);
                // println!("said_false_men: {:?}", said_false_men);
                res = tmp_res;
            }
        }
        // for value in dict_vec.values() {
        //     let first_value = value[0];
        //     for value_i in value {
        //         if *value_i != first_value {
        //             continue 'outer;
        //         }
        //     }
        // }
        // if tmp_res > res {
        //     println!("{}", i);
        //     println!("{:?}", dict_vec);
        //     res = tmp_res;
        // }
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//     }
//     // 不親切でない → 正直者の可能性がある
//     // → 不親切でないものを探す
//     // → 矛盾しない人何人いるか
//     let mut muzyun_matrix = vec![vec![-1; n]; n];
//     for i in 0..n {
//         input! {
//             a_i: usize,
//             xy_i: [(usize, i64); a_i]
//         }
//         for xy_i_j in xy_i {
//             muzyun_matrix[i][xy_i_j.0 - 1] = xy_i_j.1;
//         }
//     }
//
//     let mut muzyun_matrix2 = vec![vec![false; n]; n];
//     for i in 0..n {
//         let mut zeros = vec![];
//         let mut ones = vec![];
//         for j in 0..n {
//             if muzyun_matrix[j][i] == 0 {
//                 zeros.push(j);
//             } else if muzyun_matrix[j][i] == 1 {
//                 ones.push(j);
//             }
//         }
//         for k in 0..zeros.len() {
//             for l in 0..ones.len() {
//                 muzyun_matrix2[zeros[k]][ones[l]] = true;
//                 muzyun_matrix2[ones[l]][zeros[k]] = true;
//             }
//         }
//     }
//     // println!("{:?}", muzyun_matrix2);
//
//     let mut res_minus = 0;
//     loop {
//         let mut max_true_cnt = 0;
//         let mut max_true_cnt_i = 0;
//         for i in 0..n {
//             let mut true_cnt = 0;
//             for j in 0..n {
//                 if muzyun_matrix2[i][j] {
//                     true_cnt += 1;
//                 }
//             }
//             if true_cnt > max_true_cnt {
//                 max_true_cnt = true_cnt;
//                 max_true_cnt_i = i;
//             }
//         }
//         if max_true_cnt == 0 {
//             break;
//         } else {
//             println!("{:?}", muzyun_matrix2);
//             res_minus += 1;
//             for i in 0..n {
//                 muzyun_matrix2[max_true_cnt_i][i] = false;
//                 muzyun_matrix2[i][max_true_cnt_i] = false;
//             }
//             println!("{:?}", muzyun_matrix2);
//         }
//     }
//
//     println!("{}", n - res_minus);
// }
