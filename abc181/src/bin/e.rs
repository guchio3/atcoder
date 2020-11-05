use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i64; n],
        mut w: [i64; m],
    }
    let mut res = 9_223_372_036_854_775_807;
    h.sort();
    w.sort();

    if n == 1 {
        for j in 0..m {
            res = min(res, (h[0] - w[j]).abs());
        }
        println!("{}", res);
        return;
    }

    let mut usual_cumsums = vec![];
    let mut unusual_cumsums = vec![];
    let mut usual_cumsum = 0;
    let mut unusual_cumsum = 0;
    for i in 0..n {
        if i % 2 != 0 {
            usual_cumsum += h[i] - h[i - 1];
            unusual_cumsum += h[n - i] - h[n - i - 1];
        }
        usual_cumsums.push(usual_cumsum);
        unusual_cumsums.push(unusual_cumsum);
    }
    unusual_cumsums = unusual_cumsums.into_iter().rev().collect();

    for i in 0..n {
        let j_res = w.binary_search_by_key(&h[i], |&w_i| w_i);
        let j;
        if m == 1 {
            j = 0;
        } else {
            j = match j_res {
                Ok(jj) => jj,
                Err(jj) => {
                    if jj == 0 {
                        jj
                    } else if jj == m {
                        m - 1
                    } else {
                        if (w[jj] - h[i]).abs() > (w[jj - 1] - h[i]).abs() {
                            jj - 1
                        } else {
                            jj
                        }
                    }
                }
            };
        }
        let temp_res;
        if i % 2 == 0 {
            let l;
            if i == 0 {
                l = 0;
            } else {
                l = usual_cumsums[i - 1];
            }
            let r;
            if i == n - 1 {
                r = 0;
            } else {
                r = unusual_cumsums[i + 1];
            }
            if i == 0 {
                temp_res = 0 + unusual_cumsums[1] + (w[j] - h[i]).abs();
            } else {
                temp_res = l + r + (w[j] - h[i]).abs();
            }
        } else {
            let l;
            if i == 1 {
                l = 0;
            } else {
                l = usual_cumsums[i - 2];
            }
            let r;
            if i == n - 2 {
                r = 0;
            } else {
                r = unusual_cumsums[i + 2];
            }
            temp_res = l + r + (w[j] - h[i]).abs() + (h[i + 1] - h[i - 1]).abs();
        }
        res = min(res, temp_res);
    }
    println!("{}", res);
}


// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         mut h: [i64; n],
//         mut w: [i64; m],
//     }
//     let mut res = 9_223_372_036_854_775_807;
//     h.sort();
//     w.sort();
// 
//     if n == 1 {
//         for j in 0..m {
//             res = min(res, (h[0] - w[j]).abs());
//         }
//         println!("{}", res);
//         return;
//     }
// 
//     let mut usual_cumsums = vec![];
//     let mut unusual_cumsums = vec![];
//     let mut usual_cumsum = 0;
//     let mut unusual_cumsum = 0;
//     for i in 0..n {
//         if i % 2 != 0 {
//             usual_cumsum += h[i] - h[i - 1];
//             unusual_cumsum += h[n - i] - h[n - i - 1];
//         }
//         usual_cumsums.push(usual_cumsum);
//         unusual_cumsums.push(unusual_cumsum);
//     }
//     unusual_cumsums = unusual_cumsums.into_iter().rev().collect();
// 
//     for i in 0..n {
//         // let j = h.binary_search_by_key().unwrap();
//         for j in 0..m {
//             let temp_res;
//             if i % 2 == 0 {
//                 let l;
//                 if i == 0 {
//                     l = 0;
//                 } else {
//                     l = usual_cumsums[i - 1];
//                 }
//                 let r;
//                 if i == n - 1 {
//                     r = 0;
//                 } else {
//                     r = unusual_cumsums[i + 1];
//                 }
//                 if i == 0 {
//                     temp_res = 0 + unusual_cumsums[1] + (w[j] - h[i]).abs();
//                 } else {
//                     temp_res = l + r + (w[j] - h[i]).abs();
//                 }
//             } else {
//                 let l;
//                 if i == 1 {
//                     l = 0;
//                 } else {
//                     l = usual_cumsums[i - 2];
//                 }
//                 let r;
//                 if i == n - 2 {
//                     r = 0;
//                 } else {
//                     r = unusual_cumsums[i + 2];
//                 }
//                 temp_res = l + r + (w[j] - h[i]).abs() + (h[i + 1] - h[i - 1]).abs();
//             }
//             res = min(res, temp_res);
//         }
//     }
//     println!("{}", res);
// }
