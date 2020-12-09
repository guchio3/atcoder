#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    if n == 1 {
        println!("1");
        return;
    }
    let mut status;

    let mut res = 1;
    let mut bef_status = 0;
    let mut bef_a_i = a[0];
    let mut i = 1;
    while i < n {
        if a[i] == bef_a_i {
            status = 0;
        } else if a[i] > bef_a_i {
            status = 1;
        } else {
            status = 2;
        }

        if status == 0 || bef_status == 0 || status == bef_status {
            if bef_status == 0 {
                bef_status = status;
            }
        } else {
            res += 1;
            bef_status = 0;
        }
        bef_a_i = a[i];
        i += 1;
    }
    println!("{}", res);
}
