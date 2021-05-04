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
        a: [i64; n]
    }
    // 絶対存在するのでは？
    let mut ball_nums = vec![0; n];
    for i_plus_one in (1..=n).rev() {
        let i = i_plus_one - 1;
        let a_i = a[i];

        let mut iter_sum = 0;
        let mut iter_num = i_plus_one * 2;
        while iter_num <= n {
            iter_sum += ball_nums[iter_num - 1];
            iter_num += i_plus_one;
        }
        if iter_sum % 2 != a_i {
            ball_nums[i] = 1;
        }
    }

    let mut reses = vec![];
    for i in 0..n {
        if ball_nums[i] == 1 {
            reses.push(i + 1);
        }
    }

    println!("{}", reses.len());
    if reses.len() > 0 {
        println!("{}", reses.into_iter().join(" "));
    }
}

// fn get_prime_factors(num: i64) -> Vec<i64> {
//     let mut res = vec![];
//     let mut iter_num = num;
//     for i in 2..=num {
//         if i > iter_num {
//             break;
//         }
//         while iter_num % i == 0 {
//             res.push(i);
//             iter_num /= i;
//         }
//     }
//     res
// }
//
// fn main() {
//     input! {
//         n: usize,
//         a: [i64; n]
//     }
//
//     let mut factors_dict = HashMap::new();
//     for a_i in a {
//         let a_i_prime_factors = get_prime_factors(a_i);
//         for a_i_prime_factor_i in a_i_prime_factors {
//             *factors_dict.entry(a_i_prime_factor_i).or_insert(0) += 1;
//         }
//     }
//
//     let mut res;
//
//     if reses.len() == 0 {
//         println!("-1");
//     } else {
//         println!("{}", reses.len());
//         println!("{}", reses.iter().join(" "));
//     }
// }
