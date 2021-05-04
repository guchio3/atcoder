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
        h: usize, w: usize, k: usize,
        s: [Chars; h]
    }
    let mut points = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                points.push((i, j));
            }
        }
    }

    points.sort();
    let mut point_rows = vec![points[0].0];
    let mut bef_point_row = points[0].0;
    for i in 1..k {
        let point_row = points[i].0;
        if bef_point_row != point_row {
            point_rows.push(point_row);
            bef_point_row = point_row;
        }
    }

    let mut res = vec![vec![0; w]; h];
    let mut res_num = 1;

    let mut i = 0;
    let mut row_start = 0;
    let mut row_end = point_rows[i];
    // if point_rows.len() == 1 {
    //     row_end = h - 1;
    // }

    loop {
        let mut col_start = 0;
        let mut col_end = 0;
        while col_end != w - 1 && s[row_end][col_end] == '.' {
            col_end += 1;
        }
        while col_end < w {
            if col_end != w - 1 && s[row_end][col_end] == '.' {
                col_end += 1;
            } else {
                // if col_end == w - 1 && s[row_end][w - 1] == '.' && res_num != 1 {
                if col_end == w - 1 && s[row_end][w - 1] == '.' {
                    res_num -= 1;
                }
                for ii in row_start..=row_end {
                    for jj in col_start..=col_end {
                        res[ii][jj] = res_num;
                    }
                }
                res_num += 1;
                col_start = col_end + 1;
                col_end = col_end + 1;
            }
        }

        i += 1;
        if i > point_rows.len() {
            break;
        }
        row_start = row_end + 1;
        if i == point_rows.len() {
            row_end = h - 1;
        } else {
            row_end = point_rows[i]
        }
    }

    for i in 0..=points[points.len() - 1].0 {
        let res_i = res[i].clone();
        if res_i.contains(&0) {
            panic!();
        }
        println!("{}", res_i.into_iter().join(" "));
    }
    for _i in points[points.len() - 1].0 + 1..h {
        let res_i = res[points[points.len() - 1].0].clone();
        println!("{}", res_i.into_iter().join(" "));
    }
}

// fn main() {
//     input! {
//         h: usize, w: usize, k: usize,
//         s: [Chars; h]
//     }
//     let mut points = vec![];
//     for i in 0..h {
//         for j in 0..w {
//             if s[i][j] == '#' {
//                 points.push((i, j));
//             }
//         }
//     }
//
//     points.sort();
//     let mut point_rows = vec![points[0].0];
//     let mut bef_point_row = points[0].0;
//     for i in 1..k {
//         let point_row = points[i].0;
//         if bef_point_row != point_row {
//             point_rows.push(point_row);
//             bef_point_row = point_row;
//         }
//     }
//
//     let mut res = vec![vec![0; w]; h];
//     let mut res_num = 1;
//
//     let mut i = 0;
//     let mut row_start = 0;
//     let mut row_end = point_rows[i];
//     if point_rows.len() == 1 {
//         row_end = h - 1;
//     }
//     loop {
//         let mut col_start = 0;
//         let mut col_end = 0;
//         while col_end != w - 1 && s[row_end][col_end] == '.' {
//             col_end += 1;
//         }
//         while col_end < w {
//             if col_end != w - 1 && s[row_end][col_end] == '.' {
//                 col_end += 1;
//             } else {
//                 if col_end == w - 1 && s[row_end][w - 1] == '.' && res_num != 1 {
//                     res_num -= 1;
//                 }
//                 for ii in row_start..=row_end {
//                     for jj in col_start..=col_end {
//                         res[ii][jj] = res_num;
//                     }
//                 }
//                 res_num += 1;
//                 col_start = col_end + 1;
//                 col_end = col_end + 1;
//             }
//         }
//
//         i += 1;
//         if i > point_rows.len() {
//             break;
//         }
//         row_start = row_end + 1;
//         if i == point_rows.len() {
//             row_end = h - 1;
//         } else {
//             row_end = point_rows[i]
//         }
//     }
//
//     for i in 0..=points[points.len() - 1].0 {
//         let res_i = res[i].clone();
//         if res_i.contains(&0) {
//             panic!();
//         }
//         println!("{}", res_i.into_iter().join(" "));
//     }
//     for _i in points[points.len() - 1].0 + 1..h {
//         let res_i = res[points[points.len() - 1].0].clone();
//         println!("{}", res_i.into_iter().join(" "));
//     }
// }

// fn main() {
//     input! {
//         h: usize, w: usize, _k: usize,
//         s: [Chars; h]
//     }
//     let mut num = 1;
//     let mut res = vec![vec![0; w]; h];
//     for i in 0..h {
//         for j in 0..w {
//             if res[i][j] != 0 {
//                 continue;
//             }
//             let mut cake_cnt = 0;
//             let mut k = i;
//             let mut l = j;
//             if s[k][l] == '#' {
//                 cake_cnt += 1;
//             }
//             'outer: while k < h - 1 || l < w - 1 {
//                 k = min(k + 1, h - 1);
//                 for ll in j..=l {
//                     if s[k][ll] == '#' {
//                         if cake_cnt == 0 {
//                             cake_cnt += 1;
//                         } else {
//                             for ii in i..=k {
//                                 for jj in j..ll {
//                                     res[ii][jj] = num;
//                                 }
//                             }
//                             num += 1;
//                             break 'outer;
//                         }
//                     }
//                 }
//                 l = min(l + 1, w - 1);
//                 for kk in i..=k {
//                     if s[kk][l] == '#' {
//                         if cake_cnt == 0 {
//                             cake_cnt += 1;
//                         } else {
//                             for ii in i..kk {
//                                 for jj in j..=l {
//                                     res[ii][jj] = num;
//                                 }
//                             }
//                             num += 1;
//                             break 'outer;
//                         }
//                     }
//                 }
//             }
//             if k == h - 1 && l == w - 1 && s[h - 1][w - 1] == '.' {
//                 for ii in i..h {
//                     for jj in j..w {
//                         res[ii][jj] = num;
//                     }
//                 }
//                 num += 1;
//             }
//         }
//     }
//     for res_i in res {
//         println!("{}", res_i.iter().join(" "));
//     }
// }
