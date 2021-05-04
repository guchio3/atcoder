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
        n: usize, mut x: usize
    }
    let mut burger_nums: Vec<usize> = vec![0; n + 1];
    let mut patty_nums: Vec<usize> = vec![0; n + 1];
    burger_nums[0] = 1;
    patty_nums[0] = 1;
    for i in 1..=n {
        burger_nums[i] = burger_nums[i - 1] * 2 + 3;
        patty_nums[i] = patty_nums[i - 1] * 2 + 1;
    }
    // println!("burger_nums: {:?}", burger_nums);
    // println!("patty_nums: {:?}", patty_nums);

    let mut res = 0;
    let mut i = n;
    while x > 0 {
        // println!("{}", x);
        if x >= burger_nums[i] {
            x -= burger_nums[i];
            res += patty_nums[i];
            if x > 0 {
                x -= 1;
                res += 1;
            }
        } else {
            x -= 1;
            i -= 1;
        }
    }

    println!("{}", res);

    // let mut burger = String::from("P");
    // for _i in 0..n {
    //     burger = String::from(format!("B{}P{}B", burger, burger));
    // }

    // let burger_chars: Vec<char> = burger.chars().into_iter().rev().collect();
    // let mut res = 0;
    // for i in 0..x {
    //     if burger_chars[i] == 'P' {
    //         res += 1;
    //     }
    // }

    // println!("{}", res);
}
