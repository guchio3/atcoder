#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        k: usize, t: usize,
        a: [usize; t]
    }
    let mut bh = BinaryHeap::new();
    for a_i in a {
        bh.push(a_i);
    }

    let mut bef = None;
    let mut res = 0;
    for _ in 0..k {
        // println!("{:?} -> res: {}, bef: {:?}", bh, res, bef);
        if bh.len() == 0 {
            bef = Some(bef.unwrap() - 1);
            res += 1;
        } else {
            let mut top = bh.pop().unwrap();
            if bef != None {
                bh.push(bef.unwrap());
            }
            if top > 1 {
                top -= 1;
                bef = Some(top);
            } else {
                bef = None;
            }
        }
    }
    println!("{}", res);
}
