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
        n: i64
    }
    let mut digits = vec![0; 11];
    digits[10] = 7;
    digits[9] = 5;
    digits[8] = 3;
    let mut cnts = vec![1; 3];
    let mut res = 0;
    loop {
        let mut num = 0;
        let mut digit = 1;
        for i in 0..11 {
            num += digit * digits[10 - i];
            digit *= 10;
        }
        if num <= n {
            if cnts[0] > 0 && cnts[1] > 0 && cnts[2] > 0 {
                res += 1;
            }
        } else {
            break;
        }

        let mut i = 10;
        while digits[i] == 7 {
            cnts[2] -= 1;
            cnts[0] += 1;
            digits[i] = 3;
            i -= 1;
        }
        match digits[i] {
            3 => {
                digits[i] = 5;
                cnts[0] -= 1;
                cnts[1] += 1;
            }
            5 => {
                digits[i] = 7;
                cnts[1] -= 1;
                cnts[2] += 1
            }
            0 => {
                digits[i] = 3;
                cnts[0] += 1
            }
            _ => panic!("???"),
        };
    }
    println!("{}", res);
}
