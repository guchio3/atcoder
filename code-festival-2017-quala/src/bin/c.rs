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
        h: usize, w: usize,
        a: [Chars; h]
    }
    let mut cnt_dict = HashMap::new();
    for a_i in a {
        for a_ij in a_i {
            *cnt_dict.entry(a_ij).or_insert(0) += 1;
        }
    }

    let hw_odd_num;
    let additional_num;
    if h % 2 != 0 {
        if w % 2 != 0 {
            additional_num = h + w - 1;
            hw_odd_num = 2;
        } else {
            additional_num = w;
            hw_odd_num = 1;
        }
    } else if w % 2 != 0 {
        additional_num = h;
        hw_odd_num = 1;
    } else {
        additional_num = 0;
        hw_odd_num = 0;
    }

    let mut mod_num = 0;
    let mut odd_num = 0;
    for (_key, value) in cnt_dict {
        if value % 4 != 0 {
            mod_num += value % 4;
            if value % 2 != 0 {
                odd_num += 1;
            }
        }
    }

    if (hw_odd_num == 0 && mod_num == 0)
        || (mod_num <= additional_num
            && ((hw_odd_num == 2 && odd_num == 1) || (hw_odd_num == 1 && odd_num == 0)))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
