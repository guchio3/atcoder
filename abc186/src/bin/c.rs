#![allow(unused_imports)]
use itertools::Itertools;
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn dec_to_oct_string<T>(decimal: T) -> String
where
    T: PrimInt + NumAssign + ToString,
{
    let mut octal_string: String = String::from("");
    let mut base = decimal;
    while base > T::zero() {
        let mut tmp_string = (base % T::from(8).unwrap()).to_string();
        tmp_string.push_str(octal_string.as_str());
        octal_string = tmp_string;

        base /= T::from(8).unwrap();
    }
    octal_string
}

fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 1..=n {
        let decimal_str = i.to_string();
        let octal_str = dec_to_oct_string(i);
        if !(decimal_str.contains("7") || octal_str.contains("7")) {
            res += 1;
        }
    }
    println!("{}", res);
}
