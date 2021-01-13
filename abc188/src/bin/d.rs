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
        n: usize, c: usize,
        abc: [(usize, usize, usize); n]
    }
    let mut starts = vec![];
    let mut ends = vec![];
    for (a_i, b_i, c_i) in abc {
        starts.push((a_i, c_i));
        ends.push((b_i + 1, c_i));
    }
    starts.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    ends.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    let mut start_i = 0;
    let mut end_i = 0;
    let mut res = 0;
    let mut pay = 0;
    let mut bef_time = 0;
    loop {
        let time;
        if start_i < n {
            if end_i < n {
                if starts[start_i].0 == ends[end_i].0 {
                    time = starts[start_i].0;
                    if pay > c {
                        res += c * (time - bef_time);
                    } else {
                        res += pay * (time - bef_time);
                    }
                    while start_i < n && starts[start_i].0 == time {
                        pay += starts[start_i].1;
                        start_i += 1;
                    }
                    while end_i < n && ends[end_i].0 == time {
                        pay -= ends[end_i].1;
                        end_i += 1;
                    }
                } else if starts[start_i].0 < ends[end_i].0 {
                    time = starts[start_i].0;
                    if pay > c {
                        res += c * (time - bef_time);
                    } else {
                        res += pay * (time - bef_time);
                    }
                    while start_i < n && starts[start_i].0 == time {
                        pay += starts[start_i].1;
                        start_i += 1;
                    }
                } else {
                    time = ends[end_i].0;
                    if pay > c {
                        res += c * (time - bef_time);
                    } else {
                        res += pay * (time - bef_time);
                    }
                    while end_i < n && ends[end_i].0 == time {
                        pay -= ends[end_i].1;
                        end_i += 1;
                    }
                }
            } else {
                panic!("does not happen.");
            }
        } else if end_i < n {
            time = ends[end_i].0;
            if pay > c {
                res += c * (time - bef_time);
            } else {
                res += pay * (time - bef_time);
            }
            while end_i < n && ends[end_i].0 == time {
                pay -= ends[end_i].1;
                end_i += 1;
            }
        } else {
            break;
        }
        // if pay > c {
        //     res += c * (time - bef_time);
        // } else {
        //     res += pay * (time - bef_time);
        // }

        bef_time = time;
    }

    println!("{}", res);
}
