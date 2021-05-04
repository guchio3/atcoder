#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 1_000_000_007;

fn get_prime_factors(num: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        } else if i * i > num {
            res.push(iter_num);
            break;
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut all_cnt_dict = HashMap::new();
    for elem in get_prime_factors(a[0]) {
        *all_cnt_dict.entry(elem).or_insert(0) += 1;
    }
    let mut cnt_dicts = vec![];
    cnt_dicts.push(all_cnt_dict.clone());
    for i in 1..n {
        let a_i = a[i];
        let mut cnt_dict = HashMap::new();
        for elem in get_prime_factors(a_i) {
            *cnt_dict.entry(elem).or_insert(0) += 1;
        }
        for (key, value) in cnt_dict.iter() {
            if let Some(all_value) = all_cnt_dict.get_mut(&key) {
                if value > all_value {
                    *all_value = *value;
                }
            } else {
                all_cnt_dict.insert(*key, *value);
            }
        }
        cnt_dicts.push(cnt_dict);
    }
    for (_key, value) in all_cnt_dict.iter_mut() {
        *value *= n;
    }

    println!("{:?}", cnt_dicts);
    println!("{:?}", all_cnt_dict);

    for cnt_dict in cnt_dicts {
        for (key, value) in cnt_dict {
            let all_value = all_cnt_dict.get_mut(&key).unwrap();
            *all_value -= value;
        }
    }

    let mut res = 1;
    for (key, value) in all_cnt_dict {
        res = (res + key * value) % MOD;
    }

    println!("{}", res);
}

// // from abc156_d
// fn mod_pow(mut x: usize, mut n: usize, mod_num: usize) -> usize {
//     let mut res = 1;
//     while n > 0 {
//         if n & 1 > 0 {
//             res = (res * x) % mod_num;
//         }
//         x = (x * x) % mod_num;
//         n >>= 1;
//     }
//     res
// }
//
// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//     let mut all_lcm = 1;
//     for &a_i in a.iter() {
//         all_lcm = lcm(all_lcm, a_i) % MOD;
//     }
//
//     let mut res = 0;
//     for a_i in a {
//         let rev_mod = mod_pow(a_i, MOD - 2, MOD);
//         res = (res + all_lcm * rev_mod) % MOD;
//     }
//
//     println!("{}", res);
// }
