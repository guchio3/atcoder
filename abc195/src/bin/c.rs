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
        n: usize
    }
    let mut digit: usize = 0;
    let mut n_copy = n;
    while n_copy > 0 {
        n_copy /= 10;
        digit += 1;
    }

    let mut res: usize = 0;
    for i in 1..=digit {
        if i == digit {
            let num = n - 10usize.pow(digit as u32 - 1) + 1;
            match i {
                4 => res += 1 * num,
                5 => res += 1 * num,
                6 => res += 1 * num,
                7 => res += 2 * num,
                8 => res += 2 * num,
                9 => res += 2 * num,
                10 => res += 3 * num,
                11 => res += 3 * num,
                12 => res += 3 * num,
                13 => res += 4 * num,
                14 => res += 4 * num,
                15 => res += 4 * num,
                16 => res += 5 * num,
                _ => {}
            }
        } else {
            match i {
                4 => res += 1 * 9000,
                5 => res += 1 * 90000,
                6 => res += 1 * 900000,
                7 => res += 2 * 9000000,
                8 => res += 2 * 90000000,
                9 => res += 2 * 900000000,
                10 => res += 3 * 9000000000,
                11 => res += 3 * 90000000000,
                12 => res += 3 * 900000000000,
                13 => res += 4 * 9000000000000,
                14 => res += 4 * 90000000000000,
                15 => res += 4 * 900000000000000,
                16 => res += 5 * 9000000000000000,
                _ => {}
            }
        }
    }
    println!("{}", res);
}
